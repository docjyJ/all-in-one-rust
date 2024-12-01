// declare(strict_types=1);
// ini_set('memory_limit', '2048M');
// require __DIR__ . '/../../vendor/autoload.php';
// $container = \AIO\DependencyInjection::GetContainer();
// $dockerActionManger = $container->get(\AIO\Docker\DockerActionManager::class);
// $containerDefinitionFetcher = $container->get(\AIO\ContainerDefinitionFetcher::class);
// $id = 'nextcloud-aio-nextcloud';
// $nextcloudContainer = $containerDefinitionFetcher->GetContainerById($id);
// $isNextcloudImageOutdated = $dockerActionManger->isNextcloudImageOutdated();
// if ($isNextcloudImageOutdated === true) {
//     $dockerActionManger->sendNotification($nextcloudContainer, 'AIO is outdated!', 'Please open the AIO interface or ask an administrator to update it. If you do not want to do it manually each time, you can enable the daily backup feature from the AIO interface which automatically updates all containers.', '/notify-all.sh');
// }
