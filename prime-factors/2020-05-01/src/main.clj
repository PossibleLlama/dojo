(ns main)

(defn -main
  "I run"
  []
  (prn "Hello world!"))

(defn primes [number]
  (if (> number 1)
    [number]
    []))
