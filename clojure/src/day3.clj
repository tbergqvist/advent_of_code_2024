(ns day3
  (:require
   [clojure.string :refer [split]]))

(defn parse-mul [line]
  (->> (re-seq #"\d+" line)
       (map parse-long)
       (reduce *)))

(defn part1 [input]
  (->> (re-seq #"mul\(\d+,\d+\)" input)
       (map parse-mul) 
       (reduce +)))

(defn part2 [input]
  (->> (split input #"do\(\)")
       (map #(split % #"don't\(\)"))
       (map first)
       (map #(re-seq #"mul\(\d+,\d+\)" %))
       (flatten)
       (map parse-mul)
       (reduce +)))