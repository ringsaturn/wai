import pytest
from wai import get_tz, get_country, get_city


@pytest.mark.parametrize(
    "longitude, latitude, expected_tz",
    [
        (121.4737, 31.2304, "Asia/Shanghai"),
        (116.4074, 39.9042, "Asia/Shanghai"),
        (-122.4194, 37.7749, "America/Los_Angeles"),
        (-74.006, 40.7128, "America/New_York"),
        (0, 0, "Etc/GMT"),
    ],
)
def test_get_tz(longitude, latitude, expected_tz):
    assert get_tz(longitude, latitude) == expected_tz


@pytest.mark.parametrize(
    "longitude, latitude, expected_country",
    [
        (121.4737, 31.2304, "CN"),
        (116.4074, 39.9042, "CN"),
        (-122.4194, 37.7749, "US"),
        (-74.006, 40.7128, "US"),
        (0, 0, ""),
    ],
)
def test_get_country(longitude, latitude, expected_country):
    assert get_country(longitude, latitude) == expected_country


@pytest.mark.parametrize(
    "longitude, latitude, expected_city_name",
    [
        (121.4737, 31.2304, "Shanghai"),
        (116.4074, 39.9042, "Beijing"),
        (-122.4194, 37.7749, "San Francisco"),
        (-74.006, 40.7128, "New York City"),
        (0, 0, "Takoradi"),
    ],
)
def test_get_city(longitude, latitude, expected_city_name):
    assert get_city(longitude, latitude).name == expected_city_name
