#!/usr/bin/python3
import random
import sys
import multiprocessing


def payload(THREADS, INTERVAL):
    cp, sp = 0, 0

    its = int((INTERVAL ** 2) / THREADS)

    for i in range(its):
        rand_x = random.uniform(-1, 1)
        rand_y = random.uniform(-1, 1)

        origin_dist = rand_x ** 2 + rand_y ** 2

        if origin_dist <= 1:
            cp += 1

        sp += 1

    return cp, sp


def main():
    args = sys.argv

    if (len(args) != 3):
        print("Too few args")
        exit(1)

    INTERVALS = int(args[1])
    THREADS = int(args[2])

    cps, sps = 0, 0

    pool = multiprocessing.Pool()

    workload = [pool.apply_async(payload, args=(
        THREADS, INTERVALS)) for _ in range(THREADS)]

    nums = [ar.get() for ar in workload]
    # print(f"{nums=}")

    cps, sps = [sum(i) for i in zip(*nums)]

    assert (sps != 0)

    pi = (4.0 * cps) / sps
    print("Python pi is", pi)


if __name__ == "__main__":
    main()
