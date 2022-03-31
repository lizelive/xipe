#!/bin/sh
set -e

missing='
at
batch
m4
mailx
pax
uudecode
uuencode
'

commands='
alias
ar
awk
basename
cat
cd
chgrp
chmod
chown
cksum
cmp
comm
command
cp
crontab
csplit
cut
date
dd
df
diff
dirname
du
echo
ed
env
expand
expr
false
file
find
fold
gencat
getconf
getopts
git
grep
hash
head
iconv
id
join
kill
less
ln
locale
localedef
locate
logger
logname
ls
man
mesg
mkdir
mkfifo
mktemp
more
mv
newgrp
nice
nohup
od
paste
patch
pathchk
pr
printf
ps
pwd
read
renice
rm
rmdir
sed
sh
sleep
sort
split
strings
stty
tabs
tail
tar
tee
test
time
touch
tput
tr
true
tsort
tty
umask
unalias
uname
unexpand
uniq
unzip
wait
wc
who
write
xargs
jq
base64
bash
ssh
vim
vi
nano
dot
'

for command in $commands;
do
    if ! command -v $command > /dev/null
    then
        echo "$command could not be found"
    fi
done

echo "all good"