Math_Correlations_Pearson = Коэффициент корреляции Пирсона
    .markdown =
        # Коэффициент корреляции Пирсона

        $$\frac{"{"}\mathrm{"{"}cov{"}"}\[X, Y\]{"}"}{"{"}\sigma\_{"{"}X{"}"} \sigma\_{"{"}Y{"}"}{"}"}$$

        - {"*"}{"*"}Описание:{"*"}{"*"} Насколько векторы линейно зависимы.
        - {"*"}{"*"}Когда полезно:{"*"}{"*"} Когда важна тенденция (линейная) изменения значений, а не их абсолютные величины или масштаб. Например, для поиска схожих временных рядов по форме.
        - {"*"}{"*"}Диапазон:{"*"}{"*"} От `-1` до `1`. Где `-1` - идеальная отрицательная корреляция; `1` - идеальная положительная корреляция.

         > 
         > Пирсона

        [wikipedia.org](https://en.wikipedia.org/wiki/Pearson_correlation_coefficient)

Math_Correlations_Spearman = Spearman's rank correlation coefficient
    .markdown =
        # Spearman's rank correlation coefficient

        `correlation`

        $$\frac{"{"}\mathrm{"{"}cov{"}"}(R\[X\], R\[Y\]){"}"}{"{"}\sigma\_{"{"}R\[X\]{"}"} \sigma\_{"{"}R\[Y\]{"}"}{"}"}$$

        - {"*"}{"*"}Описание:{"*"}{"*"} Насколько векторы монотонно зависимы (основано на рангах значений, а не на самих значениях).
        - {"*"}{"*"}Когда полезно:{"*"}{"*"} Когда важна монотонная связь (не обязательно линейная), и данные могут содержать выбросы или не иметь линейной зависимости. Менее чувствительно к выбросам, чем расстояние Пирсона.
        - {"*"}{"*"}Диапазон:{"*"}{"*"} От `-1` до `1`. Где `-1` - идеальная отрицательная корреляция; `1` - идеальная положительная корреляция.

        1. [wikipedia.org](https://en.wikipedia.org/wiki/Spearman%27s_rank_correlation_coefficient)

Math_Moments_Kurtosis = Kurtosis
    .markdown =
        # Kurtosis

        $$\tilde{"{"}\mu{"}"}\_4 = \frac{"{"}\mu_4{"}"}{"{"}\mu_3^{"{"}4/2{"}"}{"}"} = \frac{"{"}\mu_4{"}"}{"{"}\sigma^4{"}"} = \frac{"{"}\operatorname{"{"}E{"}"}\[(X - \mu)^4\]{"}"}{"{"}(\operatorname{"{"}E{"}"}\[(X - \mu)^2\])^{"{"}4/2{"}"}{"}"}$$

        - {"*"}{"*"}Description{"*"}{"*"}: The fourth [standardized moment](https://en.wikipedia.org/wiki/Standardized_moment) refers to the kurtosis.
        - {"*"}{"*"}Interpretation{"*"}{"*"}:
          - `< 0`: ;
          - `= 0`: ;
          - `> 0`: .

        1. [wikipedia.org](https://en.wikipedia.org/wiki/Kurtosis)

Math_Moments_Skewness = Skewness
    .markdown =
        # Skewness

        $$\tilde{"{"}\mu{"}"}\_3 = \frac{"{"}\mu_3{"}"}{"{"}\mu_2^{"{"}3/2{"}"}{"}"} = \frac{"{"}\mu_3{"}"}{"{"}\sigma^3{"}"} = \frac{"{"}\operatorname{"{"}E{"}"}\[(X - \mu)^3\]{"}"}{"{"}(\operatorname{"{"}E{"}"}\[(X - \mu)^2\])^{"{"}3/2{"}"}{"}"}$$

        - {"*"}{"*"}Description{"*"}{"*"}: The third [standardized moment](https://en.wikipedia.org/wiki/Standardized_moment) is a measure of skewness.
        - {"*"}{"*"}Interpretation{"*"}{"*"}:
          - `< 0` ;
          - `= 0` ;
          - `> 0` .

        1. [wikipedia.org](https://en.wikipedia.org/wiki/Skewness)

