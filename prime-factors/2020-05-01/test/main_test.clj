(ns main-test
  (:use clojure.test)
  (:require [main]))

(deftest of-1-is-none
  (is (= [] (main/primes 1))))

(deftest of-2-is-2
  (is (= [2] (main/primes 2))))

(deftest of-3-is-3
  (is (= [3] (main/primes 3))))

(deftest can-pass
  (is (= true true)))
