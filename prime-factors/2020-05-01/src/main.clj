(ns main)

(defn -main
  "I run"
  []
  (prn "Hello world!"))

(defn factors-of [factorial number]
  (cond
    (> factorial (Math/sqrt number))
      (if (= number 1)
        []
        [number])
    (zero?
      (mod number factorial))
      (cons factorial (factors-of factorial (/ number factorial)))
    :else (recur (inc factorial) number)))

(defn primes [number]
  (factors-of 2 number))
