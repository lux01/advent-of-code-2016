(ns day-05.core
  (:require [digest]
            [clojure.string :as string])
  (:gen-class))

(defn password-digit?
  "Checks if the integer x "
  [door-id x]
  (string/starts-with? (digest/md5 (str door-id x))
                       "00000"))

(def integers (iterate inc 1))

(defn find-password
  [door-id]  
  (->> (pmap #(digest/md5 (str door-id %)) integers)
       (filter #(string/starts-with? % "00000"))
       (take 8)
       (map #(nth % 5))
       (apply str)))

(defn hash->digit
  [hash]
  (list (Integer/parseUnsignedInt (str (nth hash 5)) 16)
        (nth hash 6)))

(defn find-second-password
  ([door-id passwd hashes]
   (let [h (first hashes)
         hs (rest hashes)
         pwd-chars (hash->digit h)]
     (cond
       (= 8 (count passwd)) (->> passwd
                                 (vec)
                                 (sort-by first)
                                 (map second)
                                 (apply str))
       (not (contains? passwd (first pwd-chars))) (recur door-id
                                                         (assoc passwd
                                                                (first pwd-chars)
                                                                (second pwd-chars))
                                                         hs)
       :else (recur door-id passwd hs))))
  ([door-id]
   (find-second-password door-id
                         (hash-map)
                         (filter #(and (string/starts-with? % "00000")
                                       (< (Integer/parseInt (str (nth % 5)) 16) 8))
                                 (pmap #(digest/md5 (str door-id %)) integers)))))

(defn find-more-interesting-password
  [door-id]
  (->> (pmap #(digest/md5 (str door-id %)) integers)
       (filter #(and (string/starts-with? % "00000")
                     (< (Integer/parseUnsignedInt (str (nth % 5)) 16) 8)))
       (take 8)
       (reduce #(assoc %1
                       (Integer/parseUnsignedInt (str (nth %2 5)) 16)
                       (nth %2 6))
               (vec "________"))
       (apply str)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Door 1 password: " (find-password "wtnhxymk"))
  (println "Door 2 password: " (find-second-password "wtnhxymk"))
  (shutdown-agents))
