CREATE TABLE t_quiz(
  id: INTEGER PRIMARY KEY,
  quiz_description: TEXT NOT NULL,
  answer_candidate_1: TEXT NOT NULL,
  answer_candidate_2: TEXT NOT NULL,
  answer_candidate_3: TEXT NOT NULL,
  answer_candidate_4: TEXT NOT NULL,
  collect_index: INTEGER,
);