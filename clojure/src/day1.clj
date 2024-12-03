(ns day1
  (:require
   [clojure.string :refer [split-lines]]))

(defn get-distance [[first_list second_list]]
  (map #(abs (- %1 %2)) (sort first_list) (sort second_list)))

(defn create-lists [acc line]
  (->> (re-seq #"\d+" line)
       (map parse-long)
       (map conj acc)))

(defn part1 [input]
  (->> (split-lines input)
       (reduce create-lists [[], []])
       (get-distance)
       (reduce +)))

(defn list-to-count-map [list]
  (reduce
   (fn [counts, num]
     (assoc counts num (inc (get counts num 0))))
   {}
   list))

(defn get-similarity-score [[first_list second_list]]
  (let [counts (list-to-count-map second_list)]
    (map #(* % (get counts % 0)) first_list)))

(defn part2 [input]
  (->> (split-lines input)
       (reduce create-lists [[], []])
       (get-similarity-score)
       (reduce +)))