use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, hooks::use_params_map, StaticSegment};
use leptos_shopify::{get_product, get_products};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="leptos-store">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <MetaTags />
            </head>
            <body>
                <App />
                <HydrationScripts options />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_store.css" />
        <Title text="Leptos Store" />
        <Router>
            <Routes fallback=|| view! {
                <div class="min-h-screen flex items-center justify-center">
                    <div class="text-center">
                        <h1 class="text-5xl font-bold mb-4">"404"</h1>
                        <p class="text-base-content/60 mb-6">"Page not found."</p>
                        <a href="/" class="btn btn-primary">"Back to shop"</a>
                    </div>
                </div>
            }>
                <ParentRoute path=StaticSegment("") view=Layout>
                    <Route path=StaticSegment("") view=Index />
                    <Route path=leptos_router::path!("products/:handle") view=ProductPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-base-100">
            // Navbar
            <div class="navbar bg-base-100 border-b border-base-200 px-4 lg:px-8">
                <div class="navbar-start">
                    <a href="/" class="text-xl font-bold tracking-tight">"Leptos Store"</a>
                </div>
                <div class="navbar-end gap-2">
                    <button class="btn btn-ghost btn-sm">"Sign in"</button>
                    <button class="btn btn-primary btn-sm">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-9H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
                        </svg>
                        "Cart"
                    </button>
                </div>
            </div>

            // Page content
            <main class="flex-1">
                <Outlet />
            </main>

            // Footer
            <footer class="footer footer-center bg-base-200 text-base-content p-6 mt-16">
                <p class="text-sm text-base-content/60">"© 2026 Leptos Store. Built with Leptos & Shopify."</p>
            </footer>
        </div>
    }
}

/// Home page — hero banner + product grid.
#[component]
fn Index() -> impl IntoView {
    let products = Resource::new_blocking(|| (), |_| get_products(12));

    view! {
        // Hero
        <div class="hero bg-base-200 py-20 px-4">
            <div class="hero-content text-center max-w-2xl">
                <div>
                    <h1 class="text-5xl font-bold mb-4">"Welcome to Leptos Store"</h1>
                    <p class="text-base-content/70 text-lg mb-8">
                        "Discover our curated collection of products, powered by Shopify."
                    </p>
                    <a href="#products" class="btn btn-primary btn-lg">"Shop now"</a>
                </div>
            </div>
        </div>

        // Product grid
        <div id="products" class="max-w-7xl mx-auto px-4 lg:px-8 py-16">
            <h2 class="text-3xl font-bold mb-2">"All Products"</h2>
            <p class="text-base-content/60 mb-10">"Browse our full catalogue."</p>

            <Suspense fallback=|| view! { <ProductGridSkeleton /> }>
                {move || Suspend::new(async move {
                    match products.await {
                        Err(e) => view! {
                            <div role="alert" class="alert alert-error max-w-lg">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <span>"Failed to load products: " {e.to_string()}</span>
                            </div>
                        }.into_any(),
                        Ok(list) if list.is_empty() => view! {
                            <div class="text-center py-24">
                                <p class="text-base-content/50 text-lg">"No products found."</p>
                            </div>
                        }.into_any(),
                        Ok(list) => view! {
                            <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                                {list.into_iter().map(|p| {
                                    let href = format!("/products/{}", p.handle);
                                    view! {
                                        <a href=href class="card card-bordered bg-base-100 shadow-sm hover:shadow-md transition-shadow duration-200 hover:border-primary/30">
                                            // Product image
                                            <figure class="aspect-square bg-base-200 overflow-hidden">
                                                {match p.featured_image {
                                                    Some(img) => view! {
                                                        <img
                                                            src=img.url
                                                            alt=img.alt_text.unwrap_or_else(|| p.title.clone())
                                                            width=img.width.unwrap_or(800)
                                                            height=img.height.unwrap_or(800)
                                                            loading="lazy"
                                                            decoding="async"
                                                            class="w-full h-full object-cover"
                                                        />
                                                    }.into_any(),
                                                    None => view! {
                                                        <div class="w-full h-full flex items-center justify-center text-base-content/20">
                                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                                                            </svg>
                                                        </div>
                                                    }.into_any(),
                                                }}
                                            </figure>

                                            <div class="card-body p-4 gap-2">
                                                // Vendor
                                                {(!p.vendor.is_empty()).then(|| view! {
                                                    <p class="text-xs text-base-content/50 uppercase tracking-wider font-medium">{p.vendor.clone()}</p>
                                                })}

                                                // Title
                                                <h2 class="card-title text-base font-semibold leading-snug">{p.title.clone()}</h2>

                                                // Description
                                                {(!p.description.is_empty()).then(|| view! {
                                                    <p class="text-sm text-base-content/60 line-clamp-2">{p.description.clone()}</p>
                                                })}

                                                // Tags
                                                {(!p.tags.is_empty()).then(|| view! {
                                                    <div class="flex flex-wrap gap-1 mt-1">
                                                        {p.tags.iter().take(3).map(|tag| view! {
                                                            <span class="badge badge-ghost badge-sm">{tag.clone()}</span>
                                                        }).collect_view()}
                                                    </div>
                                                })}

                                                <div class="card-actions items-center justify-between mt-2">
                                                    // Price
                                                    <div>
                                                        <span class="text-lg font-bold">
                                                            {p.price_range.min_variant_price.amount.clone()}
                                                            " "
                                                            {p.price_range.min_variant_price.currency_code.clone()}
                                                        </span>
                                                        {(p.price_range.min_variant_price.amount != p.price_range.max_variant_price.amount).then(|| view! {
                                                            <span class="text-xs text-base-content/50 ml-1">
                                                                "– " {p.price_range.max_variant_price.amount.clone()}
                                                            </span>
                                                        })}
                                                    </div>

                                                    // CTA
                                                    <span class="btn btn-primary btn-sm">"View"</span>
                                                </div>
                                            </div>
                                        </a>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Individual product page at `/products/:handle`.
#[component]
fn ProductPage() -> impl IntoView {
    let params = use_params_map();
    let handle = move || params.read().get("handle").unwrap_or_default();

    let product = Resource::new_blocking(handle, |h| get_product(h));

    view! {
        <div class="max-w-5xl mx-auto px-4 lg:px-8 py-12">
            // Breadcrumb
            <div class="breadcrumbs text-sm mb-8">
                <ul>
                    <li><a href="/">"Home"</a></li>
                    <li>"Product"</li>
                </ul>
            </div>

            <Suspense fallback=|| view! { <ProductPageSkeleton /> }>
                {move || Suspend::new(async move {
                    match product.await {
                        Err(e) => view! {
                            <div role="alert" class="alert alert-error max-w-lg">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <span>"Failed to load product: " {e.to_string()}</span>
                            </div>
                        }.into_any(),
                        Ok(None) => view! {
                            <div class="text-center py-24">
                                <h2 class="text-2xl font-bold mb-2">"Product not found"</h2>
                                <p class="text-base-content/50 mb-6">"This product doesn't exist or has been removed."</p>
                                <a href="/" class="btn btn-primary">"Back to shop"</a>
                            </div>
                        }.into_any(),
                        Ok(Some(p)) => view! {
                            <Title text=p.title.clone() />
                            <div class="grid gap-12 lg:grid-cols-2">
                                // Image panel
                                <div class="rounded-2xl overflow-hidden bg-base-200 aspect-square flex items-center justify-center">
                                    {match p.featured_image {
                                        Some(img) => view! {
                                            <img
                                                src=img.url
                                                alt=img.alt_text.unwrap_or_else(|| p.title.clone())
                                                width=img.width.unwrap_or(1200)
                                                height=img.height.unwrap_or(1200)
                                                loading="eager"
                                                fetchpriority="high"
                                                decoding="sync"
                                                class="w-full h-full object-cover"
                                            />
                                        }.into_any(),
                                        None => view! {
                                            <div class="text-base-content/20">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-24 w-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                                                </svg>
                                            </div>
                                        }.into_any(),
                                    }}
                                </div>

                                // Info panel
                                <div class="flex flex-col gap-4">
                                    // Vendor + type
                                    <div class="flex items-center gap-3">
                                        {(!p.vendor.is_empty()).then(|| view! {
                                            <span class="text-sm font-medium text-base-content/50 uppercase tracking-wider">{p.vendor.clone()}</span>
                                        })}
                                        {(!p.product_type.is_empty()).then(|| view! {
                                            <span class="badge badge-outline">{p.product_type.clone()}</span>
                                        })}
                                    </div>

                                    // Title
                                    <h1 class="text-3xl font-bold leading-tight">{p.title.clone()}</h1>

                                    // Price
                                    <div class="flex items-baseline gap-2">
                                        <span class="text-3xl font-bold text-primary">
                                            {p.price_range.min_variant_price.amount.clone()}
                                            " "
                                            {p.price_range.min_variant_price.currency_code.clone()}
                                        </span>
                                        {(p.price_range.min_variant_price.amount != p.price_range.max_variant_price.amount).then(|| view! {
                                            <span class="text-base text-base-content/50">
                                                "– "
                                                {p.price_range.max_variant_price.amount.clone()}
                                                " "
                                                {p.price_range.max_variant_price.currency_code.clone()}
                                            </span>
                                        })}
                                    </div>

                                    <div class="divider my-0"></div>

                                    // Description
                                    {(!p.description.is_empty()).then(|| view! {
                                        <p class="text-base-content/70 leading-relaxed">{p.description.clone()}</p>
                                    })}

                                    // Tags
                                    {(!p.tags.is_empty()).then(|| view! {
                                        <div class="flex flex-wrap gap-2">
                                            {p.tags.iter().map(|tag| view! {
                                                <span class="badge badge-ghost">{tag.clone()}</span>
                                            }).collect_view()}
                                        </div>
                                    })}

                                    // Add to cart
                                    <div class="flex gap-3 mt-4">
                                        <button class="btn btn-primary flex-1">"Add to cart"</button>
                                        <button class="btn btn-outline btn-square" aria-label="Add to wishlist">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                                            </svg>
                                        </button>
                                    </div>

                                    // Handle / slug
                                    <p class="text-xs text-base-content/30 font-mono mt-2">
                                        "/products/" {p.handle.clone()}
                                    </p>
                                </div>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Skeleton placeholder shown while the product page is loading.
#[component]
fn ProductPageSkeleton() -> impl IntoView {
    view! {
        <div class="grid gap-12 lg:grid-cols-2">
            <div class="skeleton rounded-2xl aspect-square w-full"></div>
            <div class="flex flex-col gap-4 pt-2">
                <div class="skeleton h-3 w-24"></div>
                <div class="skeleton h-8 w-3/4"></div>
                <div class="skeleton h-8 w-32"></div>
                <div class="divider my-0"></div>
                <div class="skeleton h-4 w-full"></div>
                <div class="skeleton h-4 w-5/6"></div>
                <div class="skeleton h-4 w-4/6"></div>
                <div class="flex gap-3 mt-4">
                    <div class="skeleton h-12 flex-1"></div>
                    <div class="skeleton h-12 w-12"></div>
                </div>
            </div>
        </div>
    }
}

/// Skeleton placeholder shown while the product grid is loading.
#[component]
fn ProductGridSkeleton() -> impl IntoView {
    view! {
        <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {(0..8).map(|_| view! {
                <div class="card card-bordered bg-base-100">
                    <div class="skeleton aspect-square w-full rounded-t-2xl rounded-b-none"></div>
                    <div class="card-body p-4 gap-3">
                        <div class="skeleton h-3 w-16"></div>
                        <div class="skeleton h-4 w-3/4"></div>
                        <div class="skeleton h-3 w-full"></div>
                        <div class="skeleton h-3 w-2/3"></div>
                        <div class="flex justify-between items-center mt-2">
                            <div class="skeleton h-6 w-20"></div>
                            <div class="skeleton h-8 w-24"></div>
                        </div>
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}
