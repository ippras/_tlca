import numpy as np
from scipy.cluster.hierarchy import dendrogram, linkage
from scipy.spatial.distance import squareform
import matplotlib.pyplot as plt

# 1. Ваши данные
# labels = ['C-108(-N)', 'C-1210(-N)', 'C-1540(-N)', 'H-626(-N)', 'P-519(-N)']
labels = ["Commodity", "High linoleic", "High oleic", "High palmitic, high linoleic", "High palmitic, high oleic", "High stearic, high oleic"]
distance_matrix = np.array([
    # SN-1,2,3
    # [0.00, 0.23, 0.76, 0.09, 0.63],
    # [0.23, 0.00, 0.76, 0.21, 0.54],
    # [0.76, 0.76, 0.00, 0.76, 0.45],
    # [0.09, 0.21, 0.76, 0.00, 0.62],
    # [0.63, 0.54, 0.45, 0.62, 0.00]
    # SN-2
    # [0.00, 0.19, 0.67, 0.18, 0.55],
    # [0.19, 0.00, 0.65, 0.28, 0.56],
    # [0.67, 0.65, 0.00, 0.68, 0.47],
    # [0.18, 0.28, 0.68, 0.00, 0.64],
    # [0.55, 0.56, 0.47, 0.64, 0.00]
    [0.00      , 0.11          , 0.57       , 0.10                         , 0.54                      , 0.60                     ],
    [0.11      , 0.00          , 0.66       , 0.08                         , 0.63                      , 0.69                     ],
    [0.57      , 0.66          , 0.00       , 0.61                         , 0.08                      , 0.05                     ],
    [0.10      , 0.08          , 0.61       , 0.00                         , 0.57                      , 0.64                     ],
    [0.54      , 0.63          , 0.08       , 0.57                         , 0.00                      , 0.12                     ],
    [0.60      , 0.69          , 0.05       , 0.64                         , 0.12                      , 0.00                     ],
])

# 2. Преобразование матрицы в формат, понятный для SciPy
condensed_dist_matrix = squareform(distance_matrix)

# 3. Выполнение иерархической кластеризации
# Вы можете поменять 'single' на 'average', 'complete', 'ward' и т.д.
linkage_method = 'single'
linked = linkage(condensed_dist_matrix, method=linkage_method)

# 4. Построение и отображение дендрограммы
plt.figure(figsize=(10, 7))
dendrogram(
    linked,
    orientation='top',
    labels=labels,
    distance_sort='descending'
)

plt.title(f'Dendrogram (Linkage = {linkage_method})')
plt.ylabel('Hellinger Distance')
plt.xlabel('Sample')
plt.grid(axis='y', linestyle='--')
plt.xticks(rotation=45)
plt.tight_layout()
plt.show()