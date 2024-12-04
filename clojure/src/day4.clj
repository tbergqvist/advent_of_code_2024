(ns day4
  (:require
   [clojure.string :refer [index-of]]))

(defn map2 [f, coll]
  (map (fn [x] (map f x)) coll))

(defn map3 [f, coll]
  (map2 (fn [x] (map f x)) coll))

(defn find-positions [char, s]
  (->> (map-indexed (fn [idx ch] [idx ch]) s)
       (filter #(= (second %) char))
       (map first)))

(defn neighbours [size]
  (let [list (conj '()
                   (range 1 4)
                   (map #(* size %) (range 1 4))
                   (map #(* (inc size) %) (range 1 4))
                   (map #(* (dec size) %) (range 1 4)))]
    (apply conj list (map #(map - %) list))))

(defn find-neighbours [f, size]
  (fn [pos]
    (map2 #(+ pos %) (vec (f size)))))

(defn part1 [input]
  (let [size (inc (index-of input "\n"))]
    (->> (find-positions \X input)
         (map (find-neighbours neighbours size))
         (apply concat)
         (map2 #(nth input % 0))
         (map #(= '(\M \A \S) %))
         (filter true?)
         (count))))

(defn neighbours2 [size]
  (let [list (conj '()
                   (list (dec (- size)) 0 (inc size))
                   (list (inc (- size)) 0 (dec size)))]
    (apply conj list (map #(map - %) list))))

(defn part2 [input]
  (let [size (inc (index-of input "\n"))]
    (->> (range 0 (count input))
         (map (find-neighbours neighbours2 size))
         (map3 #(nth input % 0))
         (map2 #(= '(\M \A \S) %))
         (filter #(= 2 (count (filter true? %))))
         (count))))