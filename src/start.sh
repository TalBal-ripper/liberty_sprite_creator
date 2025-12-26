#!/usr/bin/env bash

# Проверка аргумента
if [[ -z "$1" ]]; then
    echo "Использование: $0 <директория>"
    exit 1
fi

root="$1"

# Поиск всех .rs файлов
find "$root" -type f -name "*.rs" | while read -r file; do
    echo "============================="
    echo "Файл: $file"
    echo "-----------------------------"
    cat "$file"
    echo -e "\n"
done
