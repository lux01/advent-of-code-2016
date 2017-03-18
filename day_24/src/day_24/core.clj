(ns day-24.core
  (:require [clojure.math.combinatorics :as combi])
  (:gen-class))


(defprotocol Grid
  (nodes [this]
    "Returns a sequence of all the nodes in the grid.")
  (neighbours [this current]
    "Returns a sequence of all the neighbours from the current node and the distance from the current node to the neighbour.")
  (heuristic [this node1 node2]
    "Calculates a heuristic distance between two given nodes in the grid")
  (locations [this]
    "Returns a list of all the locations of interest that need to be visited")
  (location-label [this loc]
    "Returns a keyword label for the given location of interest")
  )

(defn make-grid
  [grid-str]
  (let [grid (clojure.string/split-lines grid-str)
        x-max (count (nth grid 0))
        y-max (count grid)
        x-range (range 0 x-max)
        y-range (range 0 y-max)
        at (fn [x y] (nth (nth grid y) x))
        open? (fn [[x y]] (not= \# (at x y)))
        in-bounds? (fn [[x y]] (and (<= 0 x (dec x-max))
                                    (<= 0 y (dec y-max))))]

    (reify Grid
      (nodes [this]
        (for [x x-range
              y y-range
              :when (open? [x y])]
          [x y]))

      (neighbours [this [x y]]
        (->> [[(inc x) y] [(dec x) y] [x (inc y)] [x (dec y)]]
             (filter in-bounds?)
             (filter open?)
             (map #(vector % 1))))

      (heuristic [_ [x1 y1] [x2 y2]]
        (Math/sqrt (+ (Math/pow (- x1 x2) 2)
                      (Math/pow (- y1 y2) 2))))
      
      (locations [this]
        (for [x x-range
              y y-range
              :when (and (not= \# (at x y))
                         (not= \. (at x y)))]
          [x y]))

      (location-label [this [x y]]
        (keyword (str (at x y))))
      )))

(def test-grid
  (make-grid "###########\n#0.1.....2#\n#.#######.#\n#4.......3#\n###########"))

(def challenge-grid
  (make-grid (slurp "input.txt")))

(defn- reconstruct-path
  ([came-from current]
   (reconstruct-path came-from current []))

  ([came-from current path]
   (if (not (contains? came-from current))
     (reverse path)
     (recur came-from (get came-from current) (conj path current)))))

(defn a*
  [grid start goal]
  (let [ns (nodes grid)]
    (with-local-vars [closed #{}
                      open #{start}
                      came-from {}
                      g (assoc (zipmap ns
                                       (repeat (Double/POSITIVE_INFINITY)))
                               start 0.)
                      f (assoc (zipmap ns
                                       (repeat (Double/POSITIVE_INFINITY)))
                               start (heuristic grid start goal))
                      reached-goal false]
      (while (and (not-empty @open) (not @reached-goal))
        (let [current (apply min-key (partial get @f) @open)]
          (if (= current goal) (var-set reached-goal true)
              (do (var-set open (disj @open current))
                  (var-set closed (conj @closed current))
                  (doall
                   (for [[n dist] (neighbours grid current)
                         
                         :when (not (contains? @closed n))
                         :let [g-current (get @g current)
                               g-tentative (+ g-current dist)]
                         :when (or (not (contains? @open n))
                                   (< g-tentative (get @g n)))]
                     (do (if (not (contains? @open n))
                           (var-set open (conj @open n)))
                         (var-set came-from
                                  (assoc @came-from n current))
                         (var-set g
                                  (assoc @g n g-tentative))
                         (var-set f
                                  (assoc @f n (+ (get @g n)
                                                 (heuristic grid n goal)))))))))))
      (if @reached-goal (reconstruct-path @came-from goal)
          false))))

(defn adjacency-matrix
  "Calculates the adjacency matrix for the weighted graph representing distances between all locations of interest in the given graph."
  [grid]
  (let [locs (locations grid)
        num-locs (count locs)]
    (apply assoc {}
           (mapcat identity
                   (mapcat identity
                           (for [i (range 0 num-locs)
                                 j (range (inc i) num-locs)
                                 :let [a (nth locs i)
                                       b (nth locs j)
                                       dist (count (a* grid a b))]]
                             [[[(location-label grid a)
                                (location-label grid b)] dist]
                              [[(location-label grid b)
                                (location-label grid a)] dist]]))))))

(defn min-dist
  [adj start]
  (let [nodes (apply hash-set (flatten (map first adj)))
        rest-nodes (disj nodes start)
        perms (combi/permutations rest-nodes)]
    (->> (for [perm perms]
           [(conj perm start)
            (apply +  (map #(get adj [%1 %2]) (conj perm start) perm))])
         (apply min-key second))
    ))

(defn min-dist-loop
  [adj start]
  (let [nodes (apply hash-set (flatten (map first adj)))
        rest-nodes (disj nodes start)
        perms (combi/permutations rest-nodes)]
    (->> (for [perm perms
               :let [pvec (conj (vec perm) start)]]
           [(cons start pvec)
            (apply + (map #(get adj [%1 %2])
                          (cons start pvec)
                          pvec))])
         (apply min-key second))))

(defn -main
  [& args]
  (time
   (let [adj (adjacency-matrix challenge-grid)
         start :0]
     (println "Part 1:")
     (let [[path dist] (min-dist adj :0)]
       (println "\tMinimum steps: " dist)
       (println "\tPath: " path))
     (println "Part 2:")
     (let [[path dist] (min-dist-loop adj :0)]
       (println "\tMinimum steps: " dist)
       (println "\tPath: " path))))
  )
