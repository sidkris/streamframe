import streamframe

print("=== BASIC TEST ===")

sf = streamframe.StreamFrame(["value"], 3, 0.2)

sf.append({"value": 10})
sf.append({"value": 20})
sf.append({"value": 30})
sf.append({"value": 40})

print("mean:", sf.mean("value"))
print("variance:", sf.variance("value"))
print("last:", sf.last("value"))

print("\n=== ROLLING ===")
print("rolling_mean:", sf.rolling_mean("value"))
print("rolling_std:", sf.rolling_std("value"))
print("rolling_min:", sf.rolling_min("value"))
print("rolling_max:", sf.rolling_max("value"))

print("\n=== DERIVED ===")
print("zscore:", sf.zscore("value"))
print("ewma:", sf.ewma("value"))

print("\n=== BATCH TEST ===")

sf2 = streamframe.StreamFrame(["value"], 3, 0.2)

sf2.append_batch({
    "value": [5, 15, 25, 35, 45]
})

print("mean:", sf2.mean("value"))
print("rolling_mean:", sf2.rolling_mean("value"))
print("rolling_std:", sf2.rolling_std("value"))
print("rolling_min:", sf2.rolling_min("value"))
print("rolling_max:", sf2.rolling_max("value"))
print("zscore:", sf2.zscore("value"))
print("ewma:", sf2.ewma("value"))