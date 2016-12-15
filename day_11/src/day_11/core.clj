(ns day-11.core
  (:require [clojure.zip :as z])
  (:use clojure.pprint)
  (:gen-class))

      
(defn valid-state?
  [state]
  (and (<= 0 (:lift state) 3)
       (every? identity
               (for [i (range 0 (count (:chips state)))]
                 (or (= (nth (:chips state) i) (nth (:rtgs state) i))
                     (every? identity
                             (for [j (range 0 (count (:chips state))) :when (not= i j)]
                               (not= (nth (:chips state) i) (nth (:rtgs state) j)))))))))

(defn next-moves
  [state]
  (distinct (for [dir '(:up :down)
                  type1 '(:chips :rtgs)
                  type2 '(:chips :rtgs)
                  i (range 0 (count (:chips state)))
                  j (range 0 (count (:chips state)))
                  :when (= (get-in state [type1 i])
                           (get-in state [type2 j])
                           (:lift state))]
              {:dir dir, :moves (hash-set [type1 i] [type2 j])})))

(defn apply-move
  [state move]
  (let [update-fn (if (= :up (:dir move)) inc dec)]
    (loop [s (update state :lift update-fn)
           moves (:moves move)]
      (if (empty? moves) s
          (recur (update-in s (first moves) update-fn) (rest moves))))))

(defn next-states
  ([state]
   (filter valid-state? (map (partial apply-move state) (next-moves state))))

  ([state history]
   (filter #(every? (partial not= %) history)
           (next-states state))))



(defn rebuild-path
  ([target dp]
   (rebuild-path target dp target (list)))

  ([target dp u s]
   (if (nil? (get-in dp [u :prev]))
     {:dist (get-in dp [target :dist]),
      :path (cons u s)}
     (recur target dp (get-in dp [u :prev]) (cons u s)))))


(defn dijkstra
  ([nodes source target]
   (let [dp (-> nodes
                  (zipmap (repeat {:dist Integer/MAX_VALUE, :prev nil}))
                  (assoc-in [source :dist] 0))]
     (dijkstra nodes source target dp)))

  ([nodes source target dp]
   (let [[u & us] (sort-by (comp :dist (partial get dp)) nodes)
         dist (get-in dp [u :dist])]
     (cond
       (nil? u) (rebuild-path target dp)
       (= u target) (rebuild-path target dp)
       :else (recur us source target
                    (loop [new-dp dp
                           [v & vs] (next-states u)]
                      (if (nil? v) new-dp
                          (recur (update new-dp v
                                         #(if (< (inc dist)
                                                 (get-in new-dp [v :dist]))
                                            {:dist (inc dist), :prev u} %))
                                 vs))))))))

(defn bfs
  ([start target]
   (bfs start target
        {start {:dist 0, :prev (list)}}
        (conj clojure.lang.PersistentQueue/EMPTY start)))

  ([start target dp q]
   (if (empty? q) (get dp target)
       (let [[current & rq] q
             dist (get-in dp [current :dist])
             hist (get-in dp [current :prev])
             [new-dp new-q] (loop [ndp dp
                                   nq rq
                                   [v & vs] (next-states current
                                                         hist)]
                              (if
                                  (nil? v) (list ndp nq)
                                  (recur (if (< (inc dist)
                                                (get-in ndp [v :dist] Integer/MAX_VALUE))
                                           (assoc ndp v {:dist (inc dist),
                                                         :prev (cons current hist)})
                                           ndp)
                                         (if (< (inc dist)
                                                (get-in ndp [v :dist] Integer/MAX_VALUE))
                                           (conj nq v)
                                           nq)
                                         vs
                                         )))]
         (if (= current target) (get dp target))
         (recur start target new-dp new-q)))))

(def start-state-easy
  {:lift 0, :chips [0, 0], :rtgs [1, 2]})

(def end-state-easy
  {:lift 3, :chips [3, 3], :rtgs [3, 3]})

(def all-states-easy
  (filter valid-state?
          (distinct
           (for [floor (range 0 4)
                 m1 (range 0 4)
                 m2 (range 0 4)
                 g1 (range 0 4)
                 g2 (range 0 4)]
             {:lift floor, :chips [m1 m2], :rtgs [g1 g2]}))))

(def start-state
  {:lift 0,
   :chips [0, 2, 2, 2, 2],
   :rtgs  [0, 1, 1, 1, 1]})

(def end-state
  {:lift 3,
   :chips [3, 3, 3, 3, 3]
   :rtgs  [3, 3, 3, 3, 3]})

(def start-state-hard
  {:lift 0,
   :chips [0, 2, 2, 2, 2, 0, 0]
   :rtgs  [0, 1, 1, 1, 1, 0, 0]})

(def end-state-hard
  {:lift 3,
   :chips [3, 3, 3, 3, 3, 3, 3]
   :rtgs  [3, 3, 3, 3, 3, 3, 3]})

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Computing shortest path for easy test:")
  (pprint (bfs start-state-easy end-state-easy))
  (println "\nComputing shortest path for challenge case:")
  (pprint (bfs start-state end-state))
  (println "\nComputing shortest path for hard challenge case:")
  (pprint (bfs start-state-hard end-state-hard)))
