import streamframe.streamframe as sf
import time
import random
import os

# ============================================================
# CONFIG
# ============================================================

WINDOW_SIZE = 20
TIME_WINDOW = 10
ALPHA = 0.2
ITERATIONS = 60

# ============================================================
# INIT
# ============================================================

sf = sf.StreamFrame(
    ["latency", "errors"],
    WINDOW_SIZE,
    ALPHA,
    TIME_WINDOW
)

start_time = int(time.time())
prev_mean = None

# ============================================================
# SIMULATION
# ============================================================

def simulate_latency(t):
    base = random.gauss(100, 10)

    # Inject anomaly window
    if 20 < t < 30:
        base += random.uniform(80, 150)

    return max(base, 0)


def simulate_errors(t):
    return 1.0 if random.random() < 0.05 else 0.0


# ============================================================
# UI HELPERS
# ============================================================

def clear():
    os.system("cls" if os.name == "nt" else "clear")


def bar(value, scale=250, width=30):
    filled = int((value / scale) * width)
    filled = min(filled, width)
    return "[" + "#" * filled + "-" * (width - filled) + "]"


# ============================================================
# MAIN LOOP
# ============================================================

for i in range(ITERATIONS):
    ts = start_time + i

    latency = simulate_latency(i)
    errors = simulate_errors(i)

    sf.append({
        "latency": latency,
        "errors": errors
    }, ts)

    # ---- compute metrics ----
    rolling_mean = sf.rolling_mean("latency")
    rolling_std = sf.rolling_std("latency")
    time_mean = sf.time_mean("latency")
    ewma = sf.ewma("latency")

    global_mean = sf.mean("latency")
    variance = sf.variance("latency")

    error_rate = sf.time_mean("errors")

    # ---- z-score ----
    z = 0
    if rolling_std > 0:
        z = (latency - rolling_mean) / rolling_std

    # ---- trend direction ----
    trend = "-"
    if prev_mean is not None:
        if rolling_mean > prev_mean:
            trend = "↑"
        elif rolling_mean < prev_mean:
            trend = "↓"
    prev_mean = rolling_mean

    # ---- throughput (approx) ----
    throughput = (i + 1) / max(1, (ts - start_time + 1))

    # ---- UI ----
    clear()

    print("=== STREAMFRAME LIVE DEMO ===")
    print("Real-time feature computation over streaming data\n")

    print(f"[t={i}] latency: {latency:.2f} ms {bar(latency)}")

    print("\n--- GLOBAL (lifetime stats) ---")
    print(f"mean      : {global_mean:.2f}")
    print(f"variance  : {variance:.2f}")

    print("\n--- COUNT WINDOW (last N events) ---")
    print(f"mean      : {rolling_mean:.2f} {trend}")
    print(f"std       : {rolling_std:.2f}")

    print("\n--- TIME WINDOW (last T seconds) ---")
    print(f"time_mean : {time_mean:.2f}")

    print("\n--- TREND ---")
    print(f"ewma      : {ewma:.2f}")

    print("\n--- SYSTEM ---")
    print(f"error_rate : {error_rate:.2f}")
    print(f"throughput : ~{throughput:.2f} events/sec")

    print("\n--- SIGNALS ---")

    if z > 3:
        print("🔥 CRITICAL anomaly (z > 3)")
    elif z > 2:
        print("⚠️  anomaly detected (z > 2)")
    else:
        print("normal")

    if error_rate > 0.3:
        print("🚨 high error rate")

    print("\n--- DEBUG ---")
    print(f"z-score : {z:.2f}")

    time.sleep(0.2)