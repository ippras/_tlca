import os
import re
from pathlib import Path

# === НАСТРОЙКИ ===
LANGUAGES = ['en', 'ru']
# Список папок, которые нужно оставлять в ИД. 
# Папки, которых здесь нет, будут проигнорированы при формировании ИД.
ALLOWED_DIRS = {'Correlations', 'Metrics', 'Moments'}

def sanitize_id(name):
    """
    Преобразует строку в валидный Fluent ID.
    Разрешены только a-z, A-Z, 0-9, -, _
    ID должен начинаться с буквы.
    """
    # Заменяем все недопустимые символы на подчеркивание
    sanitized = re.sub(r'[^a-zA-Z0-9_-]', '_', name)
    
    # Убираем лишние подчеркивания подряд (например, если было два пробела)
    sanitized = re.sub(r'_+', '_', sanitized).strip('_')
    
    # Если после очистки строка пустая или начинается не с буквы, добавляем префикс
    if not sanitized or not sanitized[0].isalpha():
        sanitized = 'file_' + sanitized
        
    return sanitized

def escape_fluent_text(text):
    """
    Экранирует фигурные скобки для Fluent.
    """
    res = []
    for char in text:
        if char == '{':
            res.append('{"{"}')
        elif char == '}':
            res.append('{"}"}')
        elif char == '*':
            res.append('{"*"}')
        else:
            res.append(char)
    return "".join(res)

def create_ftl_from_dir(directory_path, output_file, allowed_dirs):
    directory = Path(directory_path)
    allowed_dirs_set = set(allowed_dirs) # Используем set для быстрого поиска
    # Проверяем, существует ли исходная директория
    if not directory.exists():
        print(f"  [Пропуск] Папка {directory_path} не найдена.")
        return
    # Создаем папку для выходного файла (например, ./ftl/), если её нет
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    with open(output_file, 'w', encoding='utf-8') as out_ftl:
        
        for file_path in directory.rglob('*'):
            if not file_path.is_file():
                continue
                
            # Пропускаем index.md
            if file_path.name.lower() == 'index.md':
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
            except UnicodeDecodeError:
                continue
                
            if not content.strip():
                continue
                
            lines = content.splitlines()
            
            # 1. Значение: первая строка файла
            first_line = escape_fluent_text(re.sub(r'#+\s+', '', lines[0].strip()))
            
            # 2. ИД: фильтруем папки и соединяем через _
            rel_path = file_path.relative_to(directory)
            
            # Получаем все папки из пути к файлу
            dir_parts = rel_path.parent.parts
            
            # Оставляем только те папки, которые есть в ALLOWED_DIRS
            filtered_dirs = [d for d in dir_parts if d in allowed_dirs_set]
            
            # Добавляем имя файла (без расширения)
            file_stem = file_path.stem
            
            # Собираем части ИД и соединяем через _
            id_parts = filtered_dirs + [file_stem]
            id_base = "_".join(id_parts)
            
            msg_id = sanitize_id(id_base)
            
            # 3. Атрибут .markdown: весь файл
            markdown_attr_lines = []
            for line in lines:
                escaped_line = escape_fluent_text(line)
                if escaped_line.strip():
                    markdown_attr_lines.append(f"        {escaped_line}")
                else:
                    markdown_attr_lines.append("") 
            
            markdown_attr = "\n".join(markdown_attr_lines)

            # Записываем в FTL
            out_ftl.write(f"Math_{msg_id} = {first_line}\n")
            out_ftl.write(f"    .markdown =\n{markdown_attr}\n\n")
            
    print(f"Готово! Файл сохранен как {output_file}")

# === ЗАПУСК ===
for lang in LANGUAGES:
    input = f'./book/{lang}/markdown'
    output = f'./ftl/{lang}.ftl'
    
    print(f"Обработка языка: {lang.upper()}...")
    create_ftl_from_dir(input, output, ALLOWED_DIRS)
