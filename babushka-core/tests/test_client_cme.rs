mod utilities;

#[cfg(test)]
mod client_cme_tests {
    use super::*;
    use redis::cluster_routing::{
        MultipleNodeRoutingInfo, Route, RoutingInfo, SingleNodeRoutingInfo, SlotAddr,
    };
    use rstest::rstest;
    use utilities::cluster::{setup_test_basics_internal, SHORT_CLUSTER_TEST_TIMEOUT};
    use utilities::*;

    fn count_primaries_and_replicas(info_replication: Vec<Vec<String>>) -> (u16, u16) {
        info_replication
            .into_iter()
            .fold((0, 0), |acc, internal_vec| {
                if internal_vec.iter().any(|str| str.contains("role:master")) {
                    (acc.0 + 1, acc.1)
                } else if internal_vec.iter().any(|str| str.contains("role:slave")) {
                    (acc.0, acc.1 + 1)
                } else {
                    (acc.0, acc.1)
                }
            })
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_send_routing_no_provided_route() {
        block_on_all(async {
            let mut test_basics = setup_test_basics_internal(TestConfiguration {
                cluster_mode: ClusterMode::Enabled,
                ..Default::default()
            })
            .await;

            let mut cmd = redis::cmd("INFO");
            cmd.arg("REPLICATION");
            let info = test_basics
                .client
                .req_packed_command(&cmd, None)
                .await
                .unwrap();
            let info = redis::from_redis_value::<Vec<Vec<String>>>(&info).unwrap();
            let (primaries, replicas) = count_primaries_and_replicas(info);
            assert_eq!(primaries, 3);
            assert_eq!(replicas, 0);
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_send_routing_to_all_primaries() {
        block_on_all(async {
            let mut test_basics = setup_test_basics_internal(TestConfiguration {
                cluster_mode: ClusterMode::Enabled,
                ..Default::default()
            })
            .await;

            let mut cmd = redis::cmd("INFO");
            cmd.arg("REPLICATION");
            let info = test_basics
                .client
                .req_packed_command(
                    &cmd,
                    Some(RoutingInfo::MultiNode(MultipleNodeRoutingInfo::AllMasters)),
                )
                .await
                .unwrap();
            let info = redis::from_redis_value::<Vec<Vec<String>>>(&info).unwrap();
            let (primaries, replicas) = count_primaries_and_replicas(info);
            assert_eq!(primaries, 3);
            assert_eq!(replicas, 0);
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_send_routing_to_all_nodes() {
        block_on_all(async {
            let mut test_basics = setup_test_basics_internal(TestConfiguration {
                cluster_mode: ClusterMode::Enabled,
                ..Default::default()
            })
            .await;

            let mut cmd = redis::cmd("INFO");
            cmd.arg("REPLICATION");
            let info = test_basics
                .client
                .req_packed_command(
                    &cmd,
                    Some(RoutingInfo::MultiNode(MultipleNodeRoutingInfo::AllNodes)),
                )
                .await
                .unwrap();
            let info = redis::from_redis_value::<Vec<Vec<String>>>(&info).unwrap();
            let (primaries, replicas) = count_primaries_and_replicas(info);
            assert_eq!(primaries, 3);
            assert_eq!(replicas, 3);
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_send_routing_by_slot_to_primary() {
        block_on_all(async {
            let mut test_basics = setup_test_basics_internal(TestConfiguration {
                cluster_mode: ClusterMode::Enabled,
                ..Default::default()
            })
            .await;

            let mut cmd = redis::cmd("INFO");
            cmd.arg("REPLICATION");
            let info = test_basics
                .client
                .req_packed_command(
                    &cmd,
                    Some(RoutingInfo::SingleNode(
                        SingleNodeRoutingInfo::SpecificNode(Route::new(0, SlotAddr::Master)),
                    )),
                )
                .await
                .unwrap();
            let info = redis::from_redis_value::<Vec<Vec<String>>>(&info).unwrap();
            let (primaries, replicas) = count_primaries_and_replicas(info);
            assert_eq!(primaries, 1);
            assert_eq!(replicas, 0);
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_send_routing_by_slot_to_replica() {
        block_on_all(async {
            let mut test_basics = setup_test_basics_internal(TestConfiguration {
                cluster_mode: ClusterMode::Enabled,
                ..Default::default()
            })
            .await;

            let mut cmd = redis::cmd("INFO");
            cmd.arg("REPLICATION");
            let info = test_basics
                .client
                .req_packed_command(
                    &cmd,
                    Some(RoutingInfo::SingleNode(
                        SingleNodeRoutingInfo::SpecificNode(Route::new(0, SlotAddr::Replica)),
                    )),
                )
                .await
                .unwrap();
            let info = redis::from_redis_value::<Vec<Vec<String>>>(&info).unwrap();
            let (primaries, replicas) = count_primaries_and_replicas(info);
            assert_eq!(primaries, 0);
            assert_eq!(replicas, 1);
        });
    }
}
