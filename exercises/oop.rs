mod blog {
    pub struct Post {
        content: String,
        state: Option<Box<dyn State>>
    }
    
    impl Post {
        pub fn new() -> Self {
            Post {
                content: String::new(),
                state: Some(Box::new(Draft {})),
            }
        }
        
        // the state pattern is demonstrated by the following implementation of public interfaces of Post
        // s.t. the core implementation is further deferred to respective methods of its field of state instance
        // s.t the implementation of public interfaces of Post remain simplistic
        pub fn request_review(&mut self) {
            // self.state = self.state.clone().request_review()
            if let Some(existing_state_boxed) = self.state.take() {
                self.state = Some(existing_state_boxed.request_review());
            }
        }
        
        pub fn approve(&mut self) {
            if let Some(existing_state_boxed) = self.state.take() {
                self.state = Some(existing_state_boxed.approve());
            }
        }

        pub fn reject(&mut self) {
            if let Some(existing_state_boxed) = self.state.take() {
                self.state = Some(existing_state_boxed.reject());
            }
        }
        
        pub fn content(&self) -> &str {
            // ""
            // converting an Option to with as_ref is a common technique to expose the Option wrapped value
            // for further use while preserving the ownership of the Option to be process not affected at all
            // with the main implementation of content processing deferred to the method of State instance, it
            // results in an speficic style of implementation where the method call of content of State instance
            // would need to take up (reference to) the original Post instance in an inversed style
            self.state.as_ref().unwrap().content(self) /* content method is called with arguement of type &Box<some_state>*/
        }

        pub fn add_text(&mut self, input_text: &str) {
            self.content.push_str(input_text);
        }

        fn split_borrow_helper(&mut self) -> (&mut String, &Option<Box<dyn State>>) {
            (&mut self.content, &self.state)
        }

        // conditional add text implementation that fulfills the functionality only for certain
        // states of a given Post (i.e. when text can only be added when the post has Draft state)
        // following the state pattern
        pub fn add_text_2(&mut self, input_text: &str) {
            // working around borrow checker ban on the following line
            // due to conflicting live reference with newly-created mutable reference
            // self.state.as_ref().unwrap().add_text(self, input_text);
            
            // workaround with splitting borrow
            // https://stackoverflow.com/questions/67154204/how-to-borrow-two-disjoint-fields-when-the-borrow-is-behind-a-method-call
            let (ref_mut_content, ref_state) = self.split_borrow_helper();
            ref_state.as_ref().unwrap().add_text(ref_mut_content, input_text);
        }
        
    }
    
    trait State {
        // the Self in self: Box<Self> is the only proper choice for parameter type annotation 
        // in this case, which would be subsituted to be one of the concrete state types that
        // would be subtype of the State trait
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        
        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn reject(self: Box<Self>) -> Box<dyn State>;
        
        // explicit lifetime annotation is needed in the following methods that takes two
        // parameters of reference type and returning a reference type to clarify the expectation 
        // that the lifetime of the returned reference of the return only bears relation to
        // the input parameter of reference to the original Post, in the sense the lifetime of the
        // returned &str will be bounded by the lifetime of the Post instance, for which the returned
        // value serves as an derived value of the Post's content
        
        // the immediate following gives a default implementation that covers the majority of the implementation
        // in the concrete subtypes of the State trait. the method specific signature form of the self parameter
        // entails that the method is imply for shared references to concrete subtypes of State. Along with Deref
        // coersion, that method implementation is automatically extended to shared references to Boxed states,
        // which is the actual arguement at call-site
        fn content<'a>(self: &Self, _post: &'a Post) -> &'a str {
            ""
        }

        fn add_text(self: &Self, _post_content: &mut String, _input_text: &str) {
        }
        
    }
    
    struct Draft {}
    
    impl State for Draft {
        fn request_review(self: Box<Draft>) -> Box<dyn State> {
            Box::new(PendingReview{pre_approved: false})
        }
        
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    
    struct PendingReview {
        pre_approved: bool,
    }
    
    impl State for PendingReview {
        fn request_review(self: Box<PendingReview>) -> Box<dyn State> {
            self
        }
        
        fn approve(self: Box<Self>) -> Box<dyn State> {
            // Box::new(Published{})
            if self.pre_approved {
                return Box::new(Published{});
            };
            Box::new(PendingReview{pre_approved: true})
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft{})
        }    
    }
    
    struct Published {}
    
    impl State for Published {
        fn request_review(self: Box<Published>) -> Box<dyn State> {
            self
        }
        
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }        

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            // the return is literally of type &String and pass the type check for
            // a &str, likely due to automatic deref coersion
            &post.content
        }
    }
}

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    post.request_review();
    assert_eq!("", post.content());

    // requires two approval to be published
    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text_2("and more rubbish");
    assert_eq!("I ate a salad for lunch today", post.content());
}