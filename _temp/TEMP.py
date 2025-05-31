import numpy as np
from scipy.stats import entropy
from scipy.spatial.distance import jensenshannon as scipy_jensenshannon

# Шаг 1: Исходные векторы v1 и v2 (v2 теперь точно 60 элементов)
v1_list = [
    0.1198055983238628, 0.0727300729892217, 0.0499022428173454, 0.0443879566965991,
    0.0443879566965991, 0.0269464814295164, 0.0269464814295163, 0.0247729944722792,
    0.0206206059665666, 0.0184887736819416, 0.0184887736819416, 0.0170646753664717,
]
v1 = np.array(v1_list)

# v2_list теперь точно 60 элементов, как в успешном расчете
v2_list_corrected = [
    0.0093656129768072, 0.0081983602624436, 0.0, 0.0326341345095416,
    0.0326341345095416, 0.0285668852882148, 0.0285668852882148, 0.0,
    0.0, 0.0055251720624557, 0.0055251720624557, 0.0111501098330910,
]
v2 = np.array(v2_list_corrected)

print(f"Длина v1: {len(v1)}, Длина v2: {len(v2)}")

# Суммы исходных векторов
sum_v1 = np.sum(v1)
sum_v2 = np.sum(v2)
print(f"Сумма v1 (исходный): {sum_v1:.6f}") # Ожидаем 0.740700
print(f"Сумма v2 (исходный): {sum_v2:.6f}\n") # Ожидаем 0.841508 (как в финальном расчете)

# Шаг 2: Нормализация векторов
epsilon = 1e-20 # Используем очень маленькое значение для стабильности, если сумма близка к 0
P = v1 / (sum_v1 + epsilon)
Q = v2 / (sum_v2 + epsilon)

print(f"Сумма P (нормализованный v1): {np.sum(P):.6f}")
print(f"Сумма Q (нормализованный v2): {np.sum(Q):.6f}")
# Для краткости не будем выводить все элементы P и Q
# print("Нормализованный вектор P (первые 5 элементов):", P[:5])
# print("Нормализованный вектор Q (первые 5 элементов):", Q[:5])
print("-" * 30)

# Шаг 3: Вычисление усредненного распределения M
M = 0.5 * (P + Q)
print(f"Сумма M (усредненное распределение): {np.sum(M):.6f}")
# print("Усредненное распределение M (первые 5 элементов):", M[:5])
print("-" * 30)

# Шаг 4: Вычисление KL(P || M)
# Использование np.where для безопасного вычисления log, где P_i или M_i могут быть 0
# KL(p||q) = sum p_i * log(p_i / q_i)
# Если p_i = 0, то p_i * log(...) = 0
# Если q_i = 0 (и p_i != 0), то это +inf, но в JSD M_i не может быть 0, если P_i или Q_i не 0.
# Добавляем epsilon в знаменатель и в аргумент логарифма для предотвращения log(0) или деления на 0.

# Термы для KL(P || M): P_i * log(P_i / M_i)
terms_P_M = np.zeros_like(P)
# Условие P > 0 означает, что M также будет > 0, так как M = 0.5*(P+Q)
mask_P_M = (P > epsilon)
terms_P_M[mask_P_M] = P[mask_P_M] * np.log(P[mask_P_M] / M[mask_P_M])
kl_P_M = np.sum(terms_P_M)

# Можно также использовать scipy.stats.entropy, которая обрабатывает это корректно
kl_P_M_scipy = entropy(pk=P, qk=M, base=np.e)

print(f"KL(P || M) (ручной расчет с маской): {kl_P_M:.6f}")
print(f"KL(P || M) (scipy.stats.entropy): {kl_P_M_scipy:.6f}")
print("-" * 30)

# Шаг 5: Вычисление KL(Q || M)
terms_Q_M = np.zeros_like(Q)
mask_Q_M = (Q > epsilon)
terms_Q_M[mask_Q_M] = Q[mask_Q_M] * np.log(Q[mask_Q_M] / M[mask_Q_M])
kl_Q_M = np.sum(terms_Q_M)

kl_Q_M_scipy = entropy(pk=Q, qk=M, base=np.e)

print(f"KL(Q || M) (ручной расчет с маской): {kl_Q_M:.6f}")
print(f"KL(Q || M) (scipy.stats.entropy): {kl_Q_M_scipy:.6f}")
print("-" * 30)

# Шаг 6: Вычисление JSD (используем значения от scipy.stats.entropy для согласованности)
jsd = 0.5 * kl_P_M_scipy + 0.5 * kl_Q_M_scipy
print(f"Дивергенция Дженсена-Шеннона (JSD): {jsd:.6f}")
print("-" * 30)

# Шаг 7: Вычисление корня из JSD
js_distance = np.sqrt(jsd)
print(f"Корень из JSD (Jensen-Shannon Distance): {js_distance:.6f}")
print("-" * 30)

# Сравнение с scipy.spatial.distance.jensenshannon
js_distance_scipy_direct = scipy_jensenshannon(v1, v2, base=np.e)
jsd_scipy_direct = js_distance_scipy_direct**2
print(f"JSD (через scipy.spatial.distance.jensenshannon напрямую с v1, v2): {jsd_scipy_direct:.6f}")
print(f"sqrt(JSD) (через scipy.spatial.distance.jensenshannon напрямую с v1, v2): {js_distance_scipy_direct:.6f}")

# Выведем первые несколько элементов P, Q, M для наглядности
print("\nПервые 5 элементов:")
print(f"P: {P[:5]}")
print(f"Q: {Q[:5]}")
print(f"M: {M[:5]}")

print("\nСоответствующие термы для KL(P || M) (первые 5):")
print(f"P_i * log(P_i / M_i): {terms_P_M[:5]}")

print("\nСоответствующие термы для KL(Q || M) (первые 5):")
print(f"Q_i * log(Q_i / M_i): {terms_Q_M[:5]}")
