import streamframe

sf = streamframe.StreamFrame(["value"], 3, 0.2)

sf.append({"value": 10})
sf.append({"value": 20})
sf.append({"value": 30})
sf.append({"value": 40})

print("mean:", sf.mean("value"))
print("rolling_mean:", sf.rolling_mean("value"))
print("rolling_std:", sf.rolling_std("value"))
print("zscore:", sf.zscore("value"))
print("ewma:", sf.ewma("value"))

print("=========")

sf.append({"value": 20})
sf.append({"value": 40})
sf.append({"value": 60})
sf.append({"value": 80})

print("mean:", sf.mean("value"))
print("rolling_mean:", sf.rolling_mean("value"))
print("rolling_std:", sf.rolling_std("value"))
print("zscore:", sf.zscore("value"))
print("ewma:", sf.ewma("value"))