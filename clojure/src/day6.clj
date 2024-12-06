(ns day6
   (:require 
    [clojure.string]))

(defn switch-direction [direction]
  (case direction
    :left :up
    :down :left
    :right :down
    :up :right))

(defn get-next-step [direction [x y] world]
  (let [[new_x new_y] (case direction
                        :left [(dec x), y]
                        :right [(inc x), y]
                        :up [x, (dec y)]
                        :down [x, (inc y)])]
      (if (= \# (get (get world new_y) new_x))
      (recur (switch-direction direction) [x y] world)
      (list direction [new_x new_y]))))

(defn travel [[x y] direction world visited] 
  (if (or (< x 0) (> x 130) (< y 0) (> y 130))
    visited
    (let [[next-direction next-step] (get-next-step direction [x y] world)]
      (recur next-step next-direction world
             (conj visited [x y]) 
             ))))

(defn travel2 [[x y] direction world visited]
  (if (or (< x 0) (> x 130) (< y 0) (> y 130))
    true
    (if (contains? visited [x y direction]) false
    (let [[next-direction next-step] (get-next-step direction [x y] world)]
      (recur next-step next-direction world
             (conj visited [x y direction])
             )))))

(defn part1 [input]
  (let [world (mapv vec (clojure.string/split-lines input))]
    (->> (travel [95 67] :up world #{})
         (count)
         (dec))))

(defn part2 [input]
  (let [world (mapv vec (clojure.string/split-lines input))]
    (->> (map (fn [i] (let [x (mod i 130)
                            y (long (/ i 130))]
                        (assoc world y (assoc (get world y) x \#)))) (range 0 16900))
         (map #(travel2 [95 67] :up % #{}))
         (filter false?)
         (count))))