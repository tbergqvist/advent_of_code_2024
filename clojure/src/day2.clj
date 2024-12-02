(ns day2
  (:require
   [clojure.string :refer [split-lines]]))

(defn is-valid-sequence [going-down]
  (fn [first, second]
    (let [distance (- first second)]
      (if going-down (< 0 distance 4) (> 0 distance -4)))))

(defn valid-plan [items]
  (every? true? (map (is-valid-sequence (> (first items) (second items))) items (rest items))))

(defn is-valid [line]
  (->> (re-seq #"\d+" line)
       (map parse-long)
       (valid-plan)))

(defn part1 [input]
  (->> (split-lines input)
       (map is-valid)
       (filter true?)
       (count)))

(defn find-combos [items]
  (as-> (range 0 (count items)) i
    (map #(concat (take % items) (drop (+ % 1) items)) i)))

(defn is-valid2 [line]
  (->> (re-seq #"\d+" line)
       (map parse-long)
       (find-combos)
       (map #(valid-plan %))
       (some true?)))

(defn part2 [input]
  (->> (split-lines input)
       (map is-valid2)
       (filter true?)
       (count)))