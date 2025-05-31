import numpy as np
from scipy.stats import pearsonr, spearmanr, wasserstein_distance, entropy
from scipy.spatial.distance import euclidean, cityblock, cosine, braycurtis, jaccard, chebyshev, minkowski, canberra
from scipy.spatial.distance import jensenshannon as scipy_jensenshannon

# Данные V1 (12 элементов)
v1_list = [
    0.1198055983238628, 0.0727300729892217, 0.0499022428173454, 0.0443879566965991,
    0.0443879566965991, 0.0269464814295164, 0.0269464814295163, 0.0247729944722792,
    0.0206206059665666, 0.0184887736819416, 0.0184887736819416, 0.0170646753664717,
]

# Данные V2 (12 элементов)
v2_list = [
    0.0093656129768072, 0.0081983602624436, 0.0, 0.0326341345095416,
    0.0326341345095416, 0.0285668852882148, 0.0285668852882148, 0.0,
    0.0, 0.0055251720624557, 0.0055251720624557, 0.0111501098330910,
]

v1 = np.array(v1_list)
v2 = np.array(v2_list)

# Нормализация для вероятностных метрик (P, Q)
sum_v1 = np.sum(v1)
sum_v2 = np.sum(v2)
P = v1 / sum_v1 if sum_v1 != 0 else np.zeros_like(v1)
Q = v2 / sum_v2 if sum_v2 != 0 else np.zeros_like(v2)

print("--- Ранее рассмотренные метрики ---")

# 1. Евклидово расстояние
euclidean_dist = euclidean(v1, v2)
print(f"1. Евклидово расстояние: {euclidean_dist:.8f}")

# 2. Манхэттенское расстояние (City Block)
manhattan_dist = cityblock(v1, v2)
print(f"2. Манхэттенское расстояние: {manhattan_dist:.8f}")

# 3. Косинусное расстояние
cosine_dist = cosine(v1, v2)
print(f"3. Косинусное расстояние: {cosine_dist:.8f}")

# 4. Расстояние Брея-Кёртиса
bray_curtis_dist = braycurtis(v1, v2)
print(f"4. Расстояние Брея-Кёртиса: {bray_curtis_dist:.8f}")

# 5. Расстояние Ружички (количественный Жаккар)
sum_min_pq = np.sum(np.minimum(v1, v2))
sum_max_pq = np.sum(np.maximum(v1, v2))
if sum_max_pq == 0:
    ruzicka_similarity = 1.0 if sum_min_pq == 0 else 0.0
else:
    ruzicka_similarity = sum_min_pq / sum_max_pq
ruzicka_dist = 1 - ruzicka_similarity
print(f"5. Расстояние Ружички (количеств. Жаккар): {ruzicka_dist:.8f}")


# 6. Дивергенция/Расстояние Дженсена-Шеннона
js_distance = scipy_jensenshannon(P, Q, base=np.e) # Используем P, Q
jsd = js_distance**2
print(f"6. Дивергенция Дженсена-Шеннона (JSD): {jsd:.8f}")
print(f"   Расстояние Дженсена-Шеннона (sqrt(JSD)): {js_distance:.8f}")

# 7. Дивергенция Кульбака-Лейблера
kl_p_q = entropy(P, Q, base=np.e)
kl_q_p = entropy(Q, P, base=np.e)
print(f"7. KL-дивергенция KL(P || Q): {kl_p_q:.8f}")
print(f"   KL-дивергенция KL(Q || P): {kl_q_p:.8f}")

print("\n--- Дополнительные метрики ---")

# 8. Расстояние на основе корреляции Пирсона
if len(np.unique(v1)) > 1 and len(np.unique(v2)) > 1:
    pearson_corr, _ = pearsonr(v1, v2)
    pearson_dist = 1 - pearson_corr
else:
    pearson_dist = np.nan
print(f"8. Расстояние Пирсона (1 - r): {pearson_dist:.8f}")

# 9. Расстояние на основе ранговой корреляции Спирмена
if len(np.unique(v1)) > 1 and len(np.unique(v2)) > 1:
    spearman_corr, _ = spearmanr(v1, v2)
    spearman_dist = 1 - spearman_corr
else:
    spearman_dist = np.nan
print(f"9. Расстояние Спирмена (1 - rho): {spearman_dist:.8f}")

# 10. Расстояние Чебышёва
chebyshev_dist = chebyshev(v1, v2)
print(f"10. Расстояние Чебышёва: {chebyshev_dist:.8f}")

# 11. Расстояние Минковского (p=3)
minkowski_p3_dist = minkowski(v1, v2, p=3)
print(f"11. Расстояние Минковского (p=3): {minkowski_p3_dist:.8f}")

# 12. Расстояние Хеллингера (ручная реализация для P, Q)
def hellinger_distance_manual(p, q):
    return (1.0 / np.sqrt(2.0)) * np.sqrt(np.sum((np.sqrt(p) - np.sqrt(q))**2))

hellinger_dist_pq = hellinger_distance_manual(P,Q) # Используем P, Q
print(f"12. Расстояние Хеллингера (для P, Q): {hellinger_dist_pq:.8f}")

# 13. Расстояние Канберры
canberra_dist = canberra(v1, v2)
print(f"13. Расстояние Канберры: {canberra_dist:.8f}")

# 14. Расстояние Вассерштейна (1D)
values_indices = np.arange(len(P))
wasserstein_dist_pq = wasserstein_distance(values_indices, values_indices, P, Q)
print(f"14. Расстояние Вассерштейна (1D для P, Q): {wasserstein_dist_pq:.8f}")

# 15. Расстояние Сёренсена-Дайса (количественное)
numerator_sd = 2 * np.sum(np.minimum(v1, v2))
denominator_sd = np.sum(v1) + np.sum(v2)
if denominator_sd == 0:
    sorensen_dice_similarity = 1.0 if numerator_sd == 0 else 0.0
else:
    sorensen_dice_similarity = numerator_sd / denominator_sd
sorensen_dice_dist = 1 - sorensen_dice_similarity
print(f"15. Расстояние Сёренсена-Дайса (количеств.): {sorensen_dice_dist:.8f}")
# Проверка: Сходство Дайса = 1 - расстояние Брея-Кёртиса.
# Расстояние Дайса = расстояние Брея-Кёртиса.
# Формула для расстояния Брея-Кёртиса: sum(|Pi-Qi|) / sum(Pi+Qi)
# Формула для расстояния Сёренсена-Дайса: 1 - (2 * sum(min(Pi,Qi)) / sum(Pi+Qi))
# Они эквивалентны: sum(|Pi-Qi|) = sum(Pi+Qi) - 2*sum(min(Pi,Qi))
# Поэтому (sum(Pi+Qi) - 2*sum(min(Pi,Qi))) / sum(Pi+Qi) = 1 - (2*sum(min(Pi,Qi)) / sum(Pi+Qi))
print(f"   (Проверка: Расстояние Сёренсена-Дайса должно быть равно Брея-Кёртиса: {bray_curtis_dist:.8f})")
