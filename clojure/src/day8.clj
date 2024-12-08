(ns day8
   (:require 
    [clojure.string :as str]))

(def SIZE 49)

(defn parse-input [input]
  (->> (str/split-lines input)
       (filter seq)
       (map-indexed (fn [y, line] (map-indexed #(conj {:symbol %2 :y y :x %1}) line)))
       (flatten)
       (filter #(not (= (% :symbol) \.)))))

(defn find-antinodes [pos1 pos2]
  (let [distance (merge-with - pos1 pos2)]
    [(merge-with + pos1 distance)
     (merge-with - pos2 distance)]))

(defn find-all-antinodes [f positions] 
  (if (empty? positions)
    []
    (let [item (first positions)]
      (conj (find-all-antinodes f (rest positions)) (map #(f item %) (rest positions))))))

(defn part1 [input]
  (->> (parse-input input)
       (group-by :symbol)
       (map (fn [[_ v]] (reduce #(conj %1 (dissoc %2 :symbol)) [] v)))
       (map (partial find-all-antinodes find-antinodes))
       (flatten)
       (filter #(and (<= 0 (:x %) SIZE) (<= 0 (:y %) SIZE)))
       (distinct)
       (count)))

(defn find-antinodes-on-line [pos distance direction]
  (let [new_pos (merge-with direction pos distance)]
    (if (and (<= 0 (new_pos :x) SIZE) (<= 0 (new_pos :y) SIZE))
      (conj (find-antinodes-on-line new_pos distance direction) new_pos)
      [])))

(defn find-antinodes-between [pos1 pos2]
  (let [distance (merge-with - pos1 pos2)]
    [pos1,
     pos2,
     (find-antinodes-on-line pos1 distance +)
     (find-antinodes-on-line pos2 distance -)]))

(defn part2 [input]
  (->> (parse-input input)
       (group-by :symbol)
       (map (fn [[_ v]] (reduce #(conj %1 (dissoc %2 :symbol)) [] v)))
       (map (partial find-all-antinodes find-antinodes-between))
       (flatten)
       (distinct)
       (count)))