// declare(strict_types=1);
// ini_set('memory_limit', '2048M');
// require __DIR__ . '/../../vendor/autoload.php';
// $container = \AIO\DependencyInjection::GetContainer();
// $dockerActionManger = $container->get(\AIO\Docker\DockerActionManager::class);
// $containerDefinitionFetcher = $container->get(\AIO\ContainerDefinitionFetcher::class);
// $id = 'nextcloud-aio-nextcloud';
// $nextcloudContainer = $containerDefinitionFetcher->GetContainerById($id);
// $df = disk_free_space(DataConst::GetDataDirectory());
// if ($df !== false && (int)$df < 1024 * 1024 * 1024 * 5) {
//     error_log("The drive that hosts the mastercontainer volume has less than 5 GB free space. Container updates and backups might not succeed due to that!");
//     $dockerActionManger->sendNotification($nextcloudContainer, 'Low on space!', 'The drive that hosts the mastercontainer volume has less than 5 GB free space. Container updates and backups might not succeed due to that!');
// }
