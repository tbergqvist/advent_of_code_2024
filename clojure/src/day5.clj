(ns day5
   (:require
    [utils :refer [map2]]
    [clojure.string :refer [split, split-lines]]))

(defn rules-to-map [rules]
  (reduce (fn [list, [key, value]]
            (assoc list key (conj (get list key #{}) value)))
          {}
          rules))

(defn parse-rules [rules]
  (->> (split-lines rules)
       (map #(split % #"\|"))
       (map2 #(parse-long %))
       (rules-to-map)))

(defn update-is-valid [parsed-rules update]
  (->> (map-indexed
        (fn [idx ch] [(take idx update) ch]) update)
       (map (fn [[list ch]] (let [number_to_find (get parsed-rules ch #{})]
                              (map #(contains? number_to_find %) list)
                              )))
       (flatten)
       (every? false?)
       ))

(defn part1 [input]
  (let [[rules, updates] (split input #"\n\n")
        parsed-rules (parse-rules rules)]
    (->> (split-lines updates)
         (map #(split % #","))
         (map2 parse-long)
         (filter #(update-is-valid parsed-rules %))
         (map #(nth % (/ (count %) 2)))
         (reduce +))))

(defn sort-list [parsed-rules]
  (fn [update]
    (->> (sort-by (fn [ch] (count (filter #(contains? (set update) %) (get parsed-rules ch)))) update)
         (reverse))))

(defn part2 [input]
  (let [[rules, updates] (split input #"\n\n")
        parsed-rules (parse-rules rules)]
    (->> (split-lines updates)
         (map #(split % #","))
         (map2 parse-long)
         (filter #(not (update-is-valid parsed-rules %)))
         (map (sort-list parsed-rules))
         (map #(nth % (/ (count %) 2)))
         (reduce +))))

(let [input (slurp "../inputs/5.txt")
      [rules, updates] (split input #"\n\n")
      parsed-rules (parse-rules rules)]
  (->> (split-lines updates)
       (map #(split % #","))
       (map2 parse-long)
       (filter #(not (update-is-valid parsed-rules %)))
       (map (sort-list parsed-rules))
       (map #(nth % (/ (count %) 2)))
       (reduce +)
       ))