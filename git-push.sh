git status
echo "Enter your message"
read message
 echo 'Enter the name of the branch:'
 read branch
git add .
git commit -m "${message}"

#if [ -n "$(git status - porcelain)" ];
#then
# echo "IT IS CLEAN"
#else
#

 echo "Pushing data to remote server!!!"
 if [[ -z ${branch}  && echo "Equal" || echo "Not equal"]] ; then
    git push -u origin main
 else
    git push -u origin "${branch}"
 fi
