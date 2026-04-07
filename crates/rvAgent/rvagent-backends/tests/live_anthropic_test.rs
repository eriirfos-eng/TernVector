// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// Quick integration test - run with: ANTHROPIC_API_KEY=sk-... cargo test -p rvagent-backends --test live_anthropic_test
use rvagent_backends::AnthropicClient;
use rvagent_core::messages::Message;
use rvagent_core::models::{resolve_model, ChatModel};

#[tokio::test]
async fn test_live_anthropic_call() {
    if std::env::var("ANTHROPIC_API_KEY").is_err() {
        eprintln!("Skipping live test: ANTHROPIC_API_KEY not set");
        return;
    }
    
    let config = resolve_model("anthropic:claude-sonnet-4-20250514");
    let client = AnthropicClient::new(config).expect("failed to create client");
    
    let messages = vec![
        Message::human("What is 2+2? Reply with just the number."),
    ];
    
    let response = client.complete(&messages).await.expect("API call failed");
    let content = response.content();
    println!("Response: {}", content);
    assert!(content.contains("4"), "Expected '4' in response, got: {}", content);
}
