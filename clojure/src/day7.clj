(ns day7
   (:require 
    [clojure.string :as str]
    [clojure.math.combinatorics :as combo]))

(defn calculate [operations numbers count]
  (if (empty? numbers)
    count
    (recur
     (rest operations)
     (drop 1 numbers)
     (case (first operations)
       :+ (+ count (first numbers))
       :* (* count (first numbers))
       :|| (parse-long (str count (first numbers))))
     )))

(defn find-calibrations [operators value numbers]
  (->> (combo/selections operators (dec (count numbers)))
       (map #(vector numbers %))
       (map #(let [[numbers operations] %]
               (calculate operations (rest numbers) (first numbers))))
       (some #(== value %))))

(defn parse-input [input]
  (->> (str/split-lines input)
       (map #(str/split % #": "))
       (map (fn [[value numbers]]
              [(parse-long value)
               (map #(parse-long %) (str/split numbers #" "))]))))

(defn part1 [input]
  (->> (parse-input input)
       (filter (fn [[value numbers]]
                 (find-calibrations '(:+ :*) value numbers)))
       (map first)
       (reduce +)))

(defn part2 [input]
  (->> (parse-input input)
       (filter (fn [[value numbers]]
                 (find-calibrations '(:+ :* :||) value numbers)))
       (map first)
       (reduce +)))