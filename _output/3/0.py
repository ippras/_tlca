import numpy as np
from scipy.cluster.hierarchy import dendrogram, linkage
from scipy.spatial.distance import squareform
import matplotlib.pyplot as plt

# 1. Определяем данные
# Названия элементов для меток на графике
labels = ['C-108(-N)', 'C-1210(-N)', 'C-1540(-N)', 'H-626(-N)', 'P-519(-N)']

# Ваша матрица расстояний Хеллингера
distance_matrix = np.array([
    [0.00, 0.23, 0.76, 0.09, 0.63],
    [0.23, 0.00, 0.76, 0.21, 0.54],
    [0.76, 0.76, 0.00, 0.76, 0.45],
    [0.09, 0.21, 0.76, 0.00, 0.62],
    [0.63, 0.54, 0.45, 0.62, 0.00]
])

# 2. Преобразуем матрицу в нужный формат
# Функция linkage() требует матрицу расстояний в "сжатом" формате (одномерный массив)
# squareform() преобразует квадратную матрицу в этот формат
condensed_dist_matrix = squareform(distance_matrix)

# 3. Создаем фигуру для трех графиков
plt.figure(figsize=(20, 6))

# --- График 1: Single Linkage (одиночная связь) ---
plt.subplot(1, 3, 1)
linked_single = linkage(condensed_dist_matrix, method='single')
dendrogram(
    linked_single,
    orientation='top',
    labels=labels,
    distance_sort='descending',
    show_leaf_counts=True
)
plt.title('Dendrogram (Linkage = single)')
plt.ylabel('Hellinger Distance')
plt.xticks(rotation=45)
plt.grid(axis='y', linestyle='--')


# --- График 2: Complete Linkage (полная связь) ---
plt.subplot(1, 3, 2)
linked_complete = linkage(condensed_dist_matrix, method='complete')
dendrogram(
    linked_complete,
    orientation='top',
    labels=labels,
    distance_sort='descending',
    show_leaf_counts=True
)
plt.title('Dendrogram (Linkage = complete)')
plt.ylabel('Hellinger Distance')
plt.xticks(rotation=45)
plt.grid(axis='y', linestyle='--')


# --- График 3: Average Linkage (средняя связь) ---
plt.subplot(1, 3, 3)
linked_average = linkage(condensed_dist_matrix, method='average')
dendrogram(
    linked_average,
    orientation='top',
    labels=labels,
    distance_sort='descending',
    show_leaf_counts=True
)
plt.title('Dendrogram (Linkage = average)')
plt.ylabel('Hellinger Distance')
plt.xticks(rotation=45)
plt.grid(axis='y', linestyle='--')

# Показываем все графики
plt.tight_layout() # Автоматически подгоняет графики, чтобы они не перекрывались
plt.show()
