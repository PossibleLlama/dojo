(ns main-test
  (:use clojure.test)
  (:require [main]))

(deftest of-1-is-none
  (is (= [] (main/primes 1))))

(deftest of-2-is-2
  (is (= [2] (main/primes 2))))

(deftest of-3-is-3
  (is (= [3] (main/primes 3))))

(deftest of-4-is-2-2
  (is (= [2 2] (main/primes 4))))

(deftest of-5-is-5
  (is (= [5] (main/primes 5))))

(deftest of-6-is-2-3
  (is (= [2 3] (main/primes 6))))

(deftest of-7-is-7
  (is (= [7] (main/primes 7))))

(deftest of-8-is-2-2-2
  (is (= [2 2 2] (main/primes 8))))

(deftest of-9-is-3-3
  (is (= [3 3] (main/primes 9))))

(deftest of-10-is-2-5
  (is (= [2 5] (main/primes 10))))

(deftest of-11-is-11
  (is (= [11] (main/primes 11))))

(deftest of-24-is-2-2-2-3
  (is (= [2 2 2 3] (main/primes 24))))

(deftest of-63-is-3-3-7
  (is (= [3 3 7] (main/primes 63))))
