// tests/integration/routes.rs

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::*;

    use crate::TEST_CAPSULEER_NAME;
    use crate::TEST_CAPSULEER_REQUEST;
    use crate::TEST_MEMBER_NAME;
    use crate::TEST_MEMBER_REQUEST;
    use crate::TEST_SKILL_NAME;
    use crate::TEST_SKILL_REQUEST;
    use crate::create_client;
    use crate::create_request;

    use api::routes::skills::SkillResponse;
    use api::routes::skills::SkillRequest;

    #[tokio::test]
    async fn test_health_check() {
        let client = create_client("test").await;
        let response = client.get("/api/health-check").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        let body_str = response.into_string().await.expect("response into string");
        assert_eq!(body_str, "\"OK\"");
    }

    #[tokio::test]
    async fn test_create_member() {
        let client = create_client("test").await;
        let status = create_request(&client, "/api/members", TEST_MEMBER_REQUEST).await;
        assert_eq!(status, Status::Created);

        let response = client.get("/api/members").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        let body_str = response.into_string().await.expect("response into string");
        let member_names: Vec<String> = serde_json::from_str(&body_str).expect("deserialize member names");
        
        assert!(member_names.contains(&TEST_MEMBER_NAME.to_string()));
    }

    #[tokio::test]
    async fn test_create_capsuleer() {
        let client = create_client("test").await;
        let status = create_request(&client, "/api/members", TEST_MEMBER_REQUEST).await;
        assert_eq!(status, Status::Created);
        
        let status = create_request(&client, "/api/capsuleers", TEST_CAPSULEER_REQUEST).await;
        assert_eq!(status, Status::Created);

        let response = client.get("/api/capsuleers").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        let body_str = response.into_string().await.expect("response into string");
        let capsuleer_names: Vec<String> = serde_json::from_str(&body_str).expect("deserialize capsuleer names");
        
        assert!(capsuleer_names.contains(&TEST_CAPSULEER_NAME.to_string()));
    }

    #[tokio::test]
    async fn test_create_capsuleer_skills() {
        let client = create_client("test").await;
        let status = create_request(&client, "/api/members", TEST_MEMBER_REQUEST).await;
        assert_eq!(status, Status::Created);
        
        let status = create_request(&client, "/api/capsuleers", TEST_CAPSULEER_REQUEST).await;
        assert_eq!(status, Status::Created);
        
        let status = create_request(&client, "/api/skills", TEST_SKILL_REQUEST).await;
        assert_eq!(status, Status::Created);

        let response = client.get("/api/skills").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        let body_str = response.into_string().await.expect("response into string");
        let skills: Vec<SkillResponse> = serde_json::from_str(&body_str).expect("deserialize skills");
        let skill = skills.iter().find(|skill| skill.name == TEST_SKILL_NAME).expect("skill not found");
        let skill_request: SkillRequest = serde_json::from_str(&TEST_SKILL_REQUEST).expect("deserialize test skill");
        assert_eq!(skill.basic, skill_request.basic);
        assert_eq!(skill.advanced, skill_request.advanced);
        assert_eq!(skill.expert, skill_request.expert);
    }
}