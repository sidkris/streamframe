import streamframe
import time

print("=== STREAM TEST ===")

sf = streamframe.StreamFrame(["value"], 3, 0.2, 5)

now = int(time.time())

sf.append({"value": 10}, now - 4)
sf.append({"value": 20}, now - 3)
sf.append({"value": 30}, now - 2)
sf.append({"value": 40}, now - 1)

print("mean:", sf.mean("value"))
print("variance:", sf.variance("value"))
print("rolling_mean:", sf.rolling_mean("value"))
print("rolling_std:", sf.rolling_std("value"))
print("time_mean:", sf.time_mean("value"))
print("ewma:", sf.ewma("value"))
print("last:", sf.last("value"))

print("\n=== BATCH TEST ===")

sf2 = streamframe.StreamFrame(["value"], 3, 0.2, 5)

timestamps = [now - 4, now - 3, now - 2, now - 1]

sf2.append_batch(
    {"value": [5, 15, 25, 35]},
    timestamps
)

print("mean:", sf2.mean("value"))
print("variance:", sf2.variance("value"))
print("rolling_mean:", sf2.rolling_mean("value"))
print("rolling_std:", sf2.rolling_std("value"))
print("time_mean:", sf2.time_mean("value"))
print("ewma:", sf2.ewma("value"))