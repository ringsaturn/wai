from citiespy import random_city
from wai import get_tz, get_country, get_city

# warmup lazy init
_ = random_city()
_ = get_tz(116.3883, 39.9289)
_ = get_country(116.3883, 39.9289)
_ = get_city(116.3883, 39.9289)


def _test_get_tz():
    city = random_city()
    _ = get_tz(city.lng, city.lat)


def test_get_tz(benchmark):
    benchmark.pedantic(_test_get_tz, rounds=15000, iterations=10)


def _test_get_country():
    city = random_city()
    _ = get_country(city.lng, city.lat)


def test_get_country(benchmark):
    benchmark.pedantic(_test_get_country, rounds=15000, iterations=10)


def _test_get_city():
    city = random_city()
    _ = get_city(city.lng, city.lat)


def test_get_city(benchmark):
    benchmark.pedantic(_test_get_city, rounds=15000, iterations=10)
