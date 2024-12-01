// use crate::auth::auth_manager::AuthManager;
//
// // namespace AIO\Middleware;
//
//
// pub struct AuthMiddleware {
// auth_manager: AuthManager,
// }
//
// impl AuthMiddleware {
// // readonly class AuthMiddleware {
// // }
//
// pub fn new(&self) -> Self {
// panic!("Not implemented"); // TODO
// //     public function __construct(
// //         private AuthManager $authManager
// //     ) {
// //     }
// }
//
// pub fn __invoke(&self, request: ServerRequestInterface, handler: RequestHandlerInterface) -> ResponseInterface {
// panic!("Not implemented"); // TODO
// //     public function __invoke(ServerRequestInterface $request, RequestHandlerInterface $handler): ResponseInterface
// //     {
// //         $publicRoutes = [
// //             '/api/auth/login',
// //             '/api/auth/getlogin',
// //             '/login',
// //             '/setup',
// //             '/',
// //         ];
// //         if(!in_array($request->getUri()->getPath(), $publicRoutes)) {
// //             if(!$this->authManager->IsAuthenticated()) {
// //                 $status = 302;
// //                 $headers = ['Location' => '/'];
// //                 $response = new Response($status, $headers);
// //                 return $response;
// //             }
// //         }
// //         $response = $handler->handle($request);
// //         return $response;
// //     }
// }
// } hdshdghgdhgdh hgshghgfhfdht
