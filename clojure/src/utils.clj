(ns utils)

(defn map2 [f, coll]
  (map (fn [x] (map f x)) coll))

(defn map3 [f, coll]
  (map2 (fn [x] (map f x)) coll))