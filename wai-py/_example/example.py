from wai import get_tz, get_country, get_city

print(get_tz(121.4737, 31.2304))
# Asia/Shanghai

print(get_country(121.4737, 31.2304))
# CN

print(get_city(121.4737, 31.2304).name)
# Shanghai
