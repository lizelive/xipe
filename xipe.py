#!/usr/bin/env python3
import click
from ruamel.yaml import YAML
import docker
from io import StringIO
from collections.abc import Iterable
import json
import shlex

yaml = YAML(typ='safe')


def flatten(l):
    for el in l:
        if isinstance(el, Iterable) and not isinstance(el, str):
            for sub in flatten(el):
                yield sub
        else:
            yield el


@click.group()
def cli():
    pass


def do_installers(env, installers, install, ordered=False):
    if ordered:
        name_to_installer = {f.__name__: f for f in installers}
        for name, stuff in install.items():
            installer = name_to_installer[name]
            yield installer(env, stuff)
    else:
        for installer in installers:
            name = installer.__name__
            if name in install:
                stuff = install[name]
                yield installer(env, stuff)
        # todo check for unused stuff


def quote(value):
    #if isinstance(value, str):
    #s = value.replace('"','\\"')
    return f'"{value}"'
    #return str(value)

def install_packages(env, stuff):
    packages = ' '.join(stuff)
    return f"RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get -y install --no-install-recommends {packages}"

def addon(env, stuff):
    # yield install_packages(["apt-utils", "curl"], env)
    for name, args in stuff.items():
        script_url = f"https://raw.githubusercontent.com/microsoft/vscode-dev-containers/main/script-library/{name}.sh"
        out_path = f"/tmp/library-scripts/{name}.sh"
        yield f"ADD {script_url} {out_path}"
        yield f"RUN bash {out_path} {' '.join(args)}"
        #yield f"RUN curl https://raw.githubusercontent.com/microsoft/vscode-dev-containers/main/script-library/{name}.sh | bash -s -- {' '.join(args)}"


def apt(env, stuff):
    packages = ' '.join(stuff)
    return f"""RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \\
    && apt-get -y install --no-install-recommends {packages} \\
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*"""


def pip(env, stuff):
    packages = ' '.join(map(quote, stuff))
    yield f"RUN python3 -m pip --disable-pip-version-check --no-cache-dir install {packages}"


# def install(env, stuff):
#     return do_installers(env, [], stuff, ordered=True)


def base(env, stuff):
    return f"FROM {stuff}"


def name(env, stuff):
    yield f'LABEL name={quote(stuff)}'


def version(env, stuff):
    yield f"LABEL version={quote(stuff)}"


def features(env, stuff):
    return "LABEL " + ' '.join([f'feature.{name}={quote(value)}' for name, value in stuff.items()])

def labels(env, stuff):
    return "LABEL " + ' '.join([f'{name}={quote(value)}' for name, value in stuff.items()])

def metadata(env, stuff):
    data = shlex.quote(json.dumps({"env":env}))
    return f"RUN echo {data} > /usr/local/etc/xipe.json"

def squash(env, stuff):
    return []

def env(env, stuff):
    return "ENV " + ' '.join([f'{name}={quote(value)}' for name, value in stuff.items()])

def user(env, stuff):
    yield f'USER {env["user"]["name"]}'
    #yield "RUN . ~/.bashrc"


def cleanup(env, stuff):
    if stuff:
        has_user = False and "user" in env
        if has_user:
            yield 'USER root'
        yield "RUN apt-get autoremove -y && apt-get clean -y && rm -rf /tmp /var/lib/apt/lists/*"
        if has_user:
            yield user(env, env['user'])

def args(env, stuff):
    for name, value in stuff.items():
        yield f"ARG {name}={quote(value)}"


def env_to_dockerfile(environment):
    dockerfile_lines = flatten(do_installers(environment, 
        [base, args, name, version, env, apt, addon, pip, squash, cleanup, user, features, labels, metadata], environment, ordered=True))
    dockerfile_content = "\n".join(dockerfile_lines)
    return dockerfile_content


@cli.command()
@click.argument("path", type=click.Path(exists=True))
def convert(path):
    """Convert env to dockerfile"""

    with open(path) as env_file:
        env = yaml.load(env_file)
        dockerfile_content = env_to_dockerfile(env)

        with open("Dockerfile", "w") as dockerfile:
            dockerfile.write(dockerfile_content)


@cli.command()
@click.argument("path", type=click.Path(exists=True))
def build(path):
    """Convert env to dockerfile"""

    with open(path) as env_file:
        env = yaml.load(env_file)
        dockerfile_content = env_to_dockerfile(env)
        with open("Dockerfile", "w") as dockerfile:
            dockerfile.write(dockerfile_content)


if __name__ == '__main__':
    cli()
