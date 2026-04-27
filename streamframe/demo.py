# DEMO WRITTEN BY AN AI MODEL

import streamframe as sf
import time
import random

print("=== STREAMFRAME REAL-TIME DEMO ===")

# Create engine
sf = sf.StreamFrame(
    ["latency", "errors"],
    window_size=20,   # last 20 events
    alpha=0.2,        # EWMA smoothing
    time_window=10    # last 10 seconds
)

start_time = int(time.time())

def simulate_latency(t):
    # Normal latency ~100ms
    base = random.gauss(100, 10)

    # Inject spike between t=20 and t=30
    if 20 < t < 30:
        base += random.uniform(80, 150)

    return max(base, 0)

def simulate_errors(t):
    # baseline low error rate
    if random.random() < 0.05:
        return 1.0
    return 0.0


for i in range(50):
    ts = start_time + i

    latency = simulate_latency(i)
    errors = simulate_errors(i)

    sf.append({
        "latency": latency,
        "errors": errors
    }, ts)

    # ---- metrics ----
    mean_latency = sf.rolling_mean("latency")
    std_latency = sf.rolling_std("latency")
    ewma_latency = sf.ewma("latency")
    time_latency = sf.time_mean("latency")

    error_rate = sf.time_mean("errors")

    # ---- anomaly detection ----
    z = 0
    if std_latency > 0:
        z = (latency - mean_latency) / std_latency

    # ---- output ----
    print(f"\n[t={i}] latency={latency:.2f} ms")

    print(f"  rolling_mean: {mean_latency:.2f}")
    print(f"  rolling_std : {std_latency:.2f}")
    print(f"  ewma        : {ewma_latency:.2f}")
    print(f"  time_mean   : {time_latency:.2f}")
    print(f"  error_rate  : {error_rate:.2f}")

    # ---- alerts ----
    if z > 2:
        print("  ⚠️  Anomaly detected (latency spike)")

    if error_rate > 0.3:
        print("  🚨 High error rate")

    # simulate real-time delay
    time.sleep(0.1)