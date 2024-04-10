from typing import List

def get_tz(lng: float, lat: float) -> str:
    """Get timezonename for location.

    It will return the first positive match.
    """
    ...

def get_tzs(lng: float, lat: float) -> List[str]:
    """Get timezonenames for location.

    It will iter all polygon and return all positive match.
    """
    ...

def get_country(lng: float, lat: float) -> str:
    """Get country name in Alpha2 for location.

    It will return the first positive match.
    """
    ...

class CityRecord(object):
    lat: float
    lng: float
    name: str
    admin1: str
    admin2: str
    cc: str

def get_city(lng: float, lat: float) -> CityRecord:
    """Get city name for location.

    It will return the first positive match.
    """
    ...
