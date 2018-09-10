(ns d04.core-test
  (:require [clojure.test :refer :all]
            [d04.core :refer :all]))

(deftest test-p1
  (testing "part 1"
    (let [cases [["aa bb cc dd ee" true],
                 ["aa bb cc dd aa" false],
                 ["aa bb cc dd aaa" true]]]
      (doseq [[in expected] cases]
        (is (part-1 in) expected)))))
