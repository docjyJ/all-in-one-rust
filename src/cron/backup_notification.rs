// declare(strict_types=1);
// ini_set('memory_limit', '2048M');
// require __DIR__ . '/../../vendor/autoload.php';
// $container = \AIO\DependencyInjection::GetContainer();
// $dockerActionManger = $container->get(\AIO\Docker\DockerActionManager::class);
// $containerDefinitionFetcher = $container->get(\AIO\ContainerDefinitionFetcher::class);
// $id = 'nextcloud-aio-nextcloud';
// $nextcloudContainer = $containerDefinitionFetcher->GetContainerById($id);
// $backupExitCode = $dockerActionManger->GetBackupcontainerExitCode();
// if ($backupExitCode === 0) {
//     if (getenv('SEND_SUCCESS_NOTIFICATIONS') === "0") {
//         error_log("Daily backup successful! Only logging successful backup and not sending backup notification since that has been disabled! You can get further info by looking at the backup logs in the AIO interface.");
//     } else {
//         $dockerActionManger->sendNotification($nextcloudContainer, 'Daily backup successful!', 'You can get further info by looking at the backup logs in the AIO interface.');
//     }
// }
// if ($backupExitCode > 0) {
//     $dockerActionManger->sendNotification($nextcloudContainer, 'Daily backup failed!', 'You can get further info by looking at the backup logs in the AIO interface.');
// }
