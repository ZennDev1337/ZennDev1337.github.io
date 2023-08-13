ignore_list=["./build.sh","./CNAME","./src"]
search_dir=./

if [ $# -ne 1 ]
  then
    echo "no commit message supplied"
    echo "or too many args supplied"
    exit 1
fi


for entry in "$search_dir"*
do
    echo "$entry"
    if [[ ${ignore_list[@]} =~ $entry ]]
    then
        echo "value found"
    else
        if [[ -f $entry ]]
        then
            rm $entry
        else
            rm -r $entry
        fi
            
fi
done

cd ./src
trunk build
cp -r ./dist/* ../
cd ..
git add .
git commit -m "$1"
git push origin main