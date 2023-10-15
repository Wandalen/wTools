
# find . -type d -name "abc"

DIR_NAME="tests"
mapfile -t directories < <(find . -type d -name $DIR_NAME)

# for dir in "${directories[@]}"
# do
#   # echo "$dir"
#   count=$(grep -o $DIR_NAME <<< "$dir" | wc -l)
#   if [ "$count" -eq 1 ]
#   then
#     echo "rm -rf $dir"
#     rm -rf $dir
#   fi
# done

for dir in "${directories[@]}"
do
  # echo "$dir"
  count=$(grep -o $DIR_NAME <<< "$dir" | wc -l)
  if [ "$count" -eq 1 ]
  then
    echo "cp -R module/core/for_each/tests/smoke_test.rs $dir/"
    cp -R module/core/for_each/tests/smoke_test.rs $dir/
    # break
  fi
done
