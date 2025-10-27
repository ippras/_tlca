Metric = { $PluralCategory ->
    *[one] Metric
    [other] Metrics
}
    .hover = Metric for comparing columns
StereospecificNumber = { $number ->
    [1] Stereospecific number 1
    [2] Stereospecific number 2
    [3] Stereospecific number 3
    *[one] Stereospecific number
    [123] Stereospecific numbers 1,2,3
    [1223] Stereospecific numbers 1,2(2,3)
    [13] Stereospecific numbers 1,3
    [many] Stereospecific numbers
}
    .abbreviation = { $number ->
        [1] SN-1
        [2] SN-2
        [3] SN-3
        [123] SN-1,2,3
        [1223] SN-1,2(2,3)
        [13] SN-1,3
        *[other] SN
    }