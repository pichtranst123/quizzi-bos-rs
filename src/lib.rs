use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, Timestamp, Balance};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONTRACT_FEE: Balance = 1_000_000_000_000_000_000_000_000; // 1 NEAR in yoctoNEAR
const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000; // 1 NEAR in yoctoNEAR

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
    rewards: Vector<Balance>, // Rewards in yoctoNEAR
    ipfs_hash: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
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
    pub fn create_quiz(&mut self, game_id: String, name: String, end_time: Timestamp, rewards: Vec<Balance>, ipfs_hash: String) {
        let total_reward: Balance = rewards.iter().map(|&near_amount| near_amount * ONE_NEAR).sum();
        assert!(env::attached_deposit() >= total_reward + CONTRACT_FEE, "Insufficient deposit");
        Promise::new(env::current_account_id()).transfer(CONTRACT_FEE);
        let rewards_in_yocto: Vector<Balance> = rewards.into_iter().map(|near_amount| near_amount * ONE_NEAR).collect();
        let quiz = Quiz {
            game_id: game_id.clone(),
            name,
            start_time: env::block_timestamp(),
            end_time,
            creator: env::signer_account_id(),
            questions: Vector::new(b"q".to_vec()),
            rewards: rewards_in_yocto,
            ipfs_hash,
        };
        self.quizzes.insert(&game_id, &quiz);
    }

    pub fn add_questions(&mut self, game_id: String, questions: Vec<Question>) {
        let mut quiz = self.quizzes.get(&game_id).expect("Quiz not found");
        for question in questions.into_iter() {
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

    pub fn get_quizzes(&self) -> Vec<Quiz> {
        self.quizzes.values().collect()
    }

    pub fn get_questions(&self, game_id: String) -> Vec<Question> {
        let quiz = self.quizzes.get(&game_id).expect("Quiz not found");
        quiz.questions.iter().collect()
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

    pub fn get_winners(&self, game_id: String) -> Vec<(AccountId, u8)> {
        let rankings = self.calculate_rankings(game_id.clone());
        rankings.into_iter().take(self.quizzes.get(&game_id).expect("Quiz not found").rewards.len()).collect()
    }

    pub fn distribute_rewards(&mut self, game_id: String) {
        let quiz = self.quizzes.get(&game_id).expect("Quiz not found");
        assert!(env::block_timestamp() > quiz.end_time, "Quiz is still ongoing");
        assert!(env::signer_account_id() == quiz.creator || env::signer_account_id() == env::current_account_id(), "Unauthorized");
        let winners = self.get_winners(game_id.clone());
        for (index, (account_id, _)) in winners.into_iter().enumerate() {
            if let Some(reward_amount) = quiz.rewards.get(index as u64) {
                Promise::new(account_id).transfer(*reward_amount);
            }
        }
    }
}
