use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, Timestamp};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONTRACT_FEE: u128 = 1_000_000_000_000_000_000_000_000; // 1 NEAR in yoctoNEAR

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct QuizContract {
  quizzes: UnorderedMap<String, Quiz>,
  responses: UnorderedMap<String, Vector<Submission>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
  question_text: String,
  possible_answers: Vec<String>,
  correct_answer: String,
  score: u8,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Quiz {
  game_id: String,
  name: String,
  start_time: Timestamp,
  end_time: Timestamp,
  creator: AccountId,
  questions: Vector<Question>,
  reward_distribution: Vec<u128>,
  number_of_winners: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Submission {
  participant: AccountId,
  responses: HashMap<u64, String>,
  submission_time: Timestamp,
}

#[near_bindgen]
impl QuizContract {
  #[init]
  pub fn new() -> Self {
    Self {
      quizzes: UnorderedMap::new(b"q"),
      responses: UnorderedMap::new(b"r"),
    }
  }

  #[payable]
  pub fn create_quiz(&mut self, game_id: String, name: String, end_time: Timestamp, reward_distribution: Vec<u128>, number_of_winners: u8) {
    let total_rewards: u128 = reward_distribution.iter().sum();
    let attached_deposit = env::attached_deposit();

    assert!(attached_deposit >= total_rewards + CONTRACT_FEE, "Not enough deposit for rewards and contract fee");
    Promise::new(env::current_account_id()).transfer(CONTRACT_FEE);

    let quiz = Quiz {
      game_id: game_id.clone(),
      name,
      start_time: env::block_timestamp(),
      end_time,
      creator: env::signer_account_id(),
      questions: Vector::new(b"q".to_vec()),
      reward_distribution,
      number_of_winners,
    };
    self.quizzes.insert(&game_id, &quiz);
  }

  pub fn add_questions(&mut self, game_id: String, questions: Vec<(String, Vec<String>, String, u8)>) {
    let mut quiz = self.quizzes.get(&game_id).expect("Quiz not found");
    for (question_text, possible_answers, correct_answer, score) in questions {
      assert!(possible_answers.contains(&correct_answer), "Correct answer must be one of the possible answers");
      let question = Question {
        question_text,
        possible_answers,
        correct_answer,
        score,
      };
      quiz.questions.push(&question);
    }
    self.quizzes.insert(&game_id, &quiz);
  }

  pub fn submit_answers(&mut self, game_id: String, responses: HashMap<u64, String>) {
    let quiz = self.quizzes.get(&game_id).expect("Quiz not found");
    assert!(env::block_timestamp() < quiz.end_time, "Quiz has ended");

    let submission = Submission {
      participant: env::signer_account_id(),
      responses,
      submission_time: env::block_timestamp(),
    };

    let mut submissions = self.responses.get(&game_id).unwrap_or_else(|| Vector::new(b"r"));
    submissions.push(&submission);
    self.responses.insert(&game_id, &submissions);
  }

  pub fn calculate_rankings(&self, game_id: String) -> Vec<(AccountId, u8)> {
    let submissions = self.responses.get(&game_id).expect("No submissions for this quiz");
    let quiz = self.quizzes.get(&game_id).expect("Quiz not found");

    let mut scores: Vec<(AccountId, u8)> = Vec::new();
    for submission in submissions.iter() {
      let mut score: u8 = 0;
      for (question_index, answer) in &submission.responses {
        let question = quiz.questions.get(*question_index).expect("Question not found");
        if answer == &question.correct_answer {
          score += question.score;
        }
      }
      scores.push((submission.participant.clone(), score));
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| submissions.iter().find(|s| s.participant == a.0).unwrap().submission_time.cmp(&submissions.iter().find(|s| s.participant == b.0).unwrap().submission_time)));
    scores
  }

  pub fn distribute_rewards(&mut self, game_id: String) {
    let quiz = self.quizzes.get(&game_id).expect("Quiz not found");
    assert!(env::block_timestamp() > quiz.end_time, "Quiz is still ongoing");
    assert!(env::signer_account_id() == quiz.creator || env::signer_account_id() == env::current_account_id(), "Unauthorized");

    let rankings = self.calculate_rankings(game_id.clone());
    for (index, reward) in quiz.reward_distribution.iter().enumerate() {
      if let Some((account_id, _)) = rankings.get(index) {
        Promise::new(account_id.clone()).transfer(*reward);
      } else {
        break;
      }
    }
  }
}