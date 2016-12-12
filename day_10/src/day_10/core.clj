(ns day-10.core
  (:gen-class))

(def test-case "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2")

(defn distribute-chips
  ([state]
   (let [full-bot (first (filter (fn
                                   [[bot-num bot-state]]
                                   (and (:low bot-state) (:high bot-state)
                                        (<= 2 (count (:holding bot-state)))))
                                 (:bot state)))]
     (if (nil? full-bot) state
         (recur (distribute-chips state full-bot)))))

  ([state full-bot]
   (let [[bot-num bot] full-bot
         [low high] (sort (:holding bot))]
     (if (and (= low 17) (= high 61))
       (println "Handling bot =" bot-num))
     (-> state
                (update-in (flatten (list (:low bot) :holding))
                           #(conj % low))
                (update-in (flatten (list (:high bot) :holding))
                           #(conj % high))
                (assoc-in [:bot bot-num :holding] (list))))))

(defn parse-instruction
  [state instr]
  (condp re-matches instr
    #"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)"
    :>> #(let [[_ bot-num lo-type lo-num hi-type hi-num] %]
           (-> state
               (assoc-in [:bot (Integer. bot-num) :low]
                         [(keyword lo-type) (Integer. lo-num)])
               (assoc-in [:bot (Integer. bot-num) :high]
                         [(keyword hi-type) (Integer. hi-num)])))
    #"value (\d+) goes to bot (\d+)"
    :>> #(let [[_ val bot-num] %]
           (-> state
               (update-in [:bot (Integer. bot-num) :holding]
                          (fn [holding] (conj holding (Integer. val))))))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [input (clojure.string/split-lines (slurp "input.txt"))
        state (reduce parse-instruction {} input)
        final-state (distribute-chips state)]
    (println (* (first (get-in final-state [:output 0 :holding]))
                (first (get-in final-state [:output 1 :holding]))
                (first (get-in final-state [:output 2 :holding]))))))
