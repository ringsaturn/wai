# `wai-py`: Where Am I

## Usage

```py
from wai import get_tz, get_country, get_city

print(get_tz(121.4737, 31.2304))
# Asia/Shanghai

print(get_country(121.4737, 31.2304))
# CN

print(get_city(121.4737, 31.2304).name)
# Shanghai
```

## Benchmark

```console
pytest tests/test_benchmark.py
================================================================= test session starts ==================================================================
platform darwin -- Python 3.12.1, pytest-8.1.1, pluggy-1.4.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/ringsaturn/Projects/wai
configfile: pyproject.toml
plugins: cov-5.0.0, socket-0.7.0, benchmark-4.0.0
collected 3 items                                                                                                                                      

tests/test_benchmark.py ...                                                                                                                      [100%]


-------------------------------------------------------------------------------------------- benchmark: 3 tests -------------------------------------------------------------------------------------------
Name (time in ns)            Min                    Max                  Mean              StdDev                Median                 IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_get_tz             724.9997 (1.0)       7,620.7973 (1.70)     1,758.6414 (1.46)     764.8621 (4.20)     1,608.3999 (1.39)     970.8980 (6.47)     4764;409      568.6207 (0.69)      15000          10
test_get_city           900.0010 (1.24)      4,479.2017 (1.0)      1,206.2755 (1.0)      182.1603 (1.0)      1,158.3965 (1.0)      150.0070 (1.0)     2495;1107      828.9980 (1.0)       15000          10
test_get_country      1,908.2974 (2.63)     15,291.6997 (3.41)     4,664.2700 (3.87)     797.6507 (4.38)     4,595.8004 (3.97)     987.4930 (6.58)     4293;298      214.3958 (0.26)      15000          10
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
================================================================== 3 passed in 1.38s ===================================================================
```

## LICENSE

- This project is under [MIT license](./LICENSE)
- Timezone Shape under [ODbL-1.0 license](https://github.com/ringsaturn/tzf-rel/blob/main/LICENSE)
