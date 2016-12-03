(ns day-03.core
  (:gen-class))

(defn input->triangle
  "Converts an input line into a list of (maybe) triangle sides."
  [input-str]
  (->> (clojure.string/split input-str #"[\s]+")
       (filter (comp not empty?))
       (map #(Integer/parseUnsignedInt %))
       (sort)))

(defn triangle?
  "Checks whether a given list of sides of a triangle is actually a valid triangle."
  [tri]
  (let [a (nth tri 0), b (nth tri 1), c (nth tri 2)]
    (> (+ a b) c)))

(defn input->vertical-triangles
  "Converts a full input string into a list of (maybe) triangle sides."
  [input]
  (->> (re-seq #"(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)"
               input)
       (map rest)
       (map #(vector (list (nth % 0) (nth % 3) (nth % 6))
                    (list (nth % 1) (nth % 4) (nth % 7))
                    (list (nth % 2) (nth % 5) (nth % 8))))
       (mapcat identity)
       (map (partial map #(Integer/parseUnsignedInt %)))
       (map sort)))

(defn count-triangles
  "Counts the number of actual triangles in the input list."
  [tri-seq]
  (count (filter triangle? tri-seq)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [input-file (slurp "input.txt")
        row-tris (map input->triangle (clojure.string/split-lines input-file))
        col-tris (input->vertical-triangles input-file)]
    (println "Number of actual (row) triangles =" (count-triangles row-tris))
    (println "Number of actual (row) triangles =" (count-triangles col-tris))))
