#!/bin/bash
FILE_PATH="$( realpath -qms "${BASH_SOURCE[0]:-$PWD}" )"
DIR_PATH="${FILE_PATH%/*}"

cat << EOF > ${DIR_PATH}/unpack.sh
#!/bin/bash
FILE_PATH="\$( realpath -qms "\${BASH_SOURCE[0]:-\$PWD}" )"
DIR_PATH="\${FILE_PATH%/*}"


EOF
for filepath in ${DIR_PATH}/*
do
  [[ "$filepath" == *.md ]] && continue
  [[ "$filepath" == *.sh ]] && continue
  echo $filepath
  cat << EOFOut >> ${DIR_PATH}/unpack.sh
head -c -1 << EOF > \${DIR_PATH}/$(basename $filepath)
$(cat $filepath)
EOF
EOFOut
done
